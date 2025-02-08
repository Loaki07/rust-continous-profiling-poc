use pprof::ProfilerGuard;
use pprof::protos::Message;
use profiling::myservice::my_service_client::MyServiceClient;
use profiling::myservice::Request;
use profiling::tasks::*;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde::Deserialize;
use log;
use env_logger;
use serde_json::json;

#[derive(Debug)]
enum TaskMessage {
    Execute { 
        task_type: String,
        response: tokio::sync::oneshot::Sender<String>,
    },
    Shutdown,
}

#[derive(Deserialize)]
struct TaskRequest {
    #[serde(rename = "type")]
    task_type: String,
}

struct TaskExecutor {
    rx: mpsc::Receiver<TaskMessage>,
}

impl TaskExecutor {
    async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                TaskMessage::Execute { task_type, response } => {
                    log::info!("Executing task: {}", task_type);
                    match self.execute_task(&task_type).await {
                        Ok(profile_id) => {
                            let _ = response.send(profile_id);
                        }
                        Err(e) => {
                            log::error!("Task execution failed: {}", e);
                        }
                    }
                }
                TaskMessage::Shutdown => {
                    log::info!("Shutting down task executor");
                    break;
                }
            }
        }
    }

    async fn execute_task(&self, task_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        let guard = ProfilerGuard::new(100).unwrap();

        match task_type {
            "cpu" => {
                log::info!("Running CPU intensive task");
                for _ in 0..2 {
                    let _ = binary_tree_sum(15);
                    let _ = fibonacci(30);
                    let _ = heavy_computation(50_000);
                }
            },
            "memory" => {
                log::info!("Running memory intensive task");
                for _ in 0..3 {
                    let _ = memory_intensive();
                    let _ = string_processing();
                    thread::sleep(Duration::from_millis(50));
                }
            },
            _ => {
                log::info!("Running mixed workload");
                let handles: Vec<_> = (0..4).map(|i| {
                    thread::spawn(move || {
                        match i {
                            0 => {
                                let _ = binary_tree_sum(15);
                                let _ = fibonacci(30);
                            },
                            1 => {
                                let _ = memory_intensive();
                            },
                            2 => {
                                let _ = vector_operations(2000);
                                let _ = hash_map_operations(1000);
                            },
                            _ => {
                                let _ = process_data_pipeline(150);
                            }
                        }
                    })
                }).collect();

                for handle in handles {
                    if let Err(e) = handle.join() {
                        log::error!("Thread panicked: {:?}", e);
                    }
                }
            }
        }

        // Get profile ID from response
        if let Ok(report) = guard.report().build() {
            if let Ok(profile) = report.pprof() {
                let mut content = Vec::new();
                profile.encode(&mut content)?;
                
                let mut client = MyServiceClient::connect("http://[::1]:50051").await?;
                let request = Request {
                    data: content,
                };
                let response = client.handle_request(request).await?;
                let profile_id = String::from_utf8_lossy(&response.into_inner().result).to_string();
                return Ok(profile_id);
            }
        }
        
        Err("Failed to generate profile".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (tx, rx) = mpsc::channel(32);
    let mut executor = TaskExecutor { rx };

    // Set up HTTP server to receive task requests
    let task_sender = tx.clone();
    let http_server = HttpServer::new(move || {
        let sender = task_sender.clone();
        App::new()
            .wrap(Cors::permissive())
            .route("/task", web::post().to(move |task: web::Json<TaskRequest>| {
                let tx = sender.clone();
                async move {
                    let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                    
                    if let Err(e) = tx.send(TaskMessage::Execute { 
                        task_type: task.task_type.clone(),
                        response: response_tx,
                    }).await {
                        log::error!("Failed to send task: {}", e);
                        return HttpResponse::InternalServerError().finish();
                    }

                    // Wait for task completion and profile ID
                    match response_rx.await {
                        Ok(profile_id) => {
                            HttpResponse::Ok().json(json!({
                                "profileId": profile_id
                            }))
                        }
                        Err(_) => HttpResponse::InternalServerError().json(json!({
                            "error": "Task execution failed"
                        }))
                    }
                }
            }))
    })
    .bind("[::1]:3001")?
    .run();

    // Handle shutdown
    let shutdown_tx = tx;
    ctrlc::set_handler(move || {
        let tx = shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(TaskMessage::Shutdown).await;
        });
    })?;

    // Run both the executor and HTTP server
    tokio::select! {
        _ = executor.run() => log::info!("Executor shutdown"),
        _ = http_server => log::info!("HTTP server shutdown"),
    }

    Ok(())
} 