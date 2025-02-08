//! Profile Server
//! 
//! This server provides functionality to:
//! 1. Receive pprof profile data via gRPC
//! 2. Process and convert profiles into flame graph compatible JSON
//! 3. Store profiles in memory and on disk
//! 4. Serve profile data via HTTP API
//! 
//! The server runs two services:
//! - gRPC server on [::1]:50051 for receiving profiles
//! - HTTP server on [::1]:3000 for serving processed profiles

use tonic::{transport::Server, Request, Response, Status};
use profiling::myservice::my_service_server::{MyService, MyServiceServer};
use profiling::myservice::{Request as MyRequest, Response as MyResponse};
use pprof::protos::{Profile, Message};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use tokio::sync::RwLock;
use log;
use env_logger;
use std::time::Instant;
use ctrlc;
use std::time::Duration;
use serde::Serialize;
use std::io::Write;

/// Store for holding processed profiles in memory
/// Maps profile IDs to their JSON representations
type ProfileStore = Arc<RwLock<HashMap<String, serde_json::Value>>>;

/// gRPC service implementation for receiving profiles
#[derive(Default)]
pub struct MyServiceImpl {
    profiles: ProfileStore,
}

#[tonic::async_trait]
impl MyService for MyServiceImpl {
    /// Handles incoming profile requests
    /// 
    /// # Steps
    /// 1. Decodes pprof data
    /// 2. Processes profile into JSON
    /// 3. Stores in memory and on disk
    /// 4. Returns unique profile ID
    async fn handle_request(
        &self,
        request: Request<MyRequest>,
    ) -> Result<Response<MyResponse>, Status> {
        let start_time = Instant::now();
        let data = request.into_inner().data;
        
        let process_result = tokio::time::timeout(
            Duration::from_secs(30),
            tokio::task::spawn_blocking(move || {
                Profile::decode(&data[..]).map(|profile| {
                    let flame_data = FlameGraphData::from_profile(&profile);
                    (profile, flame_data)
                })
            })
        ).await;

        match process_result {
            Ok(Ok(Ok((profile, flame_data)))) => {
                let profile_id = uuid::Uuid::new_v4().to_string();
                
                // Store processed data
                self.profiles.write().await.insert(profile_id.clone(), json!(flame_data));

                // Save raw profile
                let mut raw_file = File::create(format!("profile_{}.pb", profile_id))
                    .map_err(|e| Status::internal(e.to_string()))?;
                let mut buf = Vec::new();
                profile.encode(&mut buf)
                    .map_err(|e| Status::internal(e.to_string()))?;
                raw_file.write_all(&buf)
                    .map_err(|e| Status::internal(e.to_string()))?;

                // Save processed data
                let json_file = File::create(format!("profile_{}.json", profile_id))
                    .map_err(|e| Status::internal(e.to_string()))?;
                serde_json::to_writer(json_file, &flame_data)
                    .map_err(|e| Status::internal(e.to_string()))?;

                log::info!("Profile ID: {}, total time: {:?}", profile_id, start_time.elapsed());
                
                Ok(Response::new(MyResponse {
                    result: profile_id.into_bytes()
                }))
            }
            Ok(Ok(Err(_))) => {
                Err(Status::invalid_argument("Invalid profile data"))
            }
            Ok(Err(_)) => {
                Err(Status::internal("Profile processing failed"))
            }
            Err(_) => {
                Err(Status::deadline_exceeded("Profile processing timed out"))
            }
        }
    }
}

/// HTTP handler for retrieving processed profiles
/// 
/// # Arguments
/// * `id` - Profile ID from URL path
/// * `profiles` - Shared store of processed profiles
/// 
/// # Returns
/// * JSON response with profile data or 404 error
async fn get_profile(
    id: web::Path<String>,
    profiles: web::Data<ProfileStore>,
) -> HttpResponse {
    log::info!("HTTP GET request for profile ID: {}", id);
    
    if let Some(profile) = profiles.read().await.get(&*id) {
        log::info!("Found profile {}, returning data", id);
        HttpResponse::Ok().json(profile)
    } else {
        log::warn!("Profile {} not found", id);
        HttpResponse::NotFound().json(json!({"error": "Profile not found"}))
    }
}

#[derive(Serialize, Debug, Clone)]
struct FlameGraphNode {
    id: String,
    name: String,
    value: u64,
    children: Vec<FlameGraphNode>
}

#[derive(Serialize, Debug, Clone)]
struct FlameGraphData {
    name: String,
    value: u64,
    children: Vec<FlameGraphNode>
}

impl FlameGraphData {
    fn from_profile(profile: &Profile) -> Self {
        let mut function_samples: HashMap<u64, u64> = HashMap::new();
        let mut function_children: HashMap<u64, HashSet<u64>> = HashMap::new();
        
        // Create location to function ID lookup
        let location_to_function: HashMap<u64, u64> = profile.location.iter()
            .filter_map(|loc| {
                loc.line.first().map(|line| (loc.id, line.function_id))
            })
            .collect();

        // Process samples and maintain full call stacks
        for sample in &profile.sample {
            // Process entire call stack
            let stack: Vec<_> = sample.location_id.iter()
                .filter_map(|&loc_id| location_to_function.get(&loc_id))
                .copied()
                .collect();

            // Update sample counts
            for &func_id in &stack {
                *function_samples.entry(func_id).or_default() += sample.value[0] as u64;
            }

            // Build parent-child relationships from the stack
            for window in stack.windows(2) {
                let [caller_id, callee_id] = window else { continue };
                function_children.entry(*caller_id)
                    .or_default()
                    .insert(*callee_id);
            }
        }

        // Find root functions (those never called by others)
        let called_functions: HashSet<_> = function_children.values()
            .flat_map(|children| children.iter())
            .copied()
            .collect();

        let root_functions: Vec<_> = profile.function.iter()
            .filter(|f| !called_functions.contains(&f.id))
            .map(|f| f.id)
            .collect();

        // Build tree
        let total_value = profile.sample.iter().map(|s| s.value[0] as u64).sum();
        let children = root_functions.iter()
            .filter_map(|&id| {
                let func = profile.function.iter().find(|f| f.id == id)?;
                let name = profile.string_table.get(func.name as usize)
                    .unwrap_or(&"unknown".to_string()).to_string();
                let value = *function_samples.get(&id).unwrap_or(&0);

                Some(FlameGraphNode {
                    id: id.to_string(),
                    name,
                    value,
                    children: build_children(id, profile, &function_samples, &function_children),
                })
            })
            .collect();

        FlameGraphData {
            name: "root".to_string(),
            value: total_value,
            children
        }
    }
}

fn build_children(
    func_id: u64,
    profile: &Profile,
    function_samples: &HashMap<u64, u64>,
    function_children: &HashMap<u64, HashSet<u64>>,
) -> Vec<FlameGraphNode> {
    let mut result = Vec::new();
    let mut stack = vec![(func_id, 0)];  // (func_id, depth)
    let mut visited = HashSet::new();

    while let Some((current_id, depth)) = stack.pop() {
        // Prevent infinite recursion and limit depth
        if depth > 50 || !visited.insert(current_id) {
            continue;
        }

        if let Some(children) = function_children.get(&current_id) {
            for &child_id in children {
                if let Some(func) = profile.function.iter().find(|f| f.id == child_id) {
                    let name = profile.string_table.get(func.name as usize)
                        .unwrap_or(&"unknown".to_string()).to_string();
                    let value = *function_samples.get(&child_id).unwrap_or(&0);

                    let node = FlameGraphNode {
                        id: child_id.to_string(),
                        name,
                        value,
                        children: Vec::new(),  // Will be filled in next iterations
                    };

                    result.push(node);
                    stack.push((child_id, depth + 1));
                }
            }
        }
    }

    result
}

/// Main entry point
/// 
/// Sets up:
/// 1. Logging
/// 2. Shared profile store
/// 3. gRPC server for receiving profiles
/// 4. HTTP server for serving profiles
/// 5. Graceful shutdown handling
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let profiles: ProfileStore = Arc::new(RwLock::new(HashMap::new()));
    let grpc_profiles = profiles.clone();

    // Start gRPC server
    let grpc_addr = "[::1]:50051".parse().unwrap();
    log::info!("gRPC server listening on {}", grpc_addr);
    
    let grpc_server = tokio::spawn(async move {
        Server::builder()
            .add_service(MyServiceServer::new(MyServiceImpl { profiles: grpc_profiles }))
            .serve(grpc_addr)
            .await
            .unwrap()
    });

    // Start HTTP server
    log::info!("HTTP server listening on [::1]:3000");
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(web::Data::new(profiles.clone()))
            .route("/api/profiles/{id}", web::get().to(get_profile))
    })
    .bind("[::1]:3000")?
    .workers(1)
    .shutdown_timeout(5)
    .run();

    // Handle shutdown gracefully
    let (tx, rx) = tokio::sync::oneshot::channel();
    let tx = Arc::new(std::sync::Mutex::new(Some(tx)));
    let tx_clone = tx.clone();

    ctrlc::set_handler(move || {
        log::info!("Received shutdown signal");
        if let Some(tx) = tx_clone.lock().unwrap().take() {
            let _ = tx.send(());
        }
    }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    tokio::select! {
        _ = grpc_server => log::info!("gRPC server terminated"),
        result = http_server => {
            if let Err(e) = result {
                log::error!("HTTP server error: {}", e);
                return Err(e.into());
            }
        }
        _ = rx => {
            log::info!("Shutting down gracefully");
            // Force exit after a timeout
            tokio::spawn(async {
                tokio::time::sleep(Duration::from_secs(6)).await;
                log::warn!("Forcing shutdown after timeout");
                std::process::exit(0);
            });
        }
    }

    Ok(())
} 