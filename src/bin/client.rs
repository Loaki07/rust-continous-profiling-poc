use pprof::ProfilerGuard;
use pprof::protos::Message;
use profiling::myservice::my_service_client::MyServiceClient;
use profiling::myservice::Request;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use log;
use env_logger;

// Recursive tree-like computation
fn binary_tree_sum(depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let left = binary_tree_sum(depth - 1);
    let right = binary_tree_sum(depth - 1);
    left + right + depth as u64
}

// String manipulation intensive
fn string_processing() -> String {
    let mut result = String::with_capacity(1000);
    for i in 0..100 {
        result.push_str(&format!("Processing item {}: ", i));
        result.push_str(&fibonacci(i % 15).to_string());
        result.push('\n');
    }
    result
}

// Hash map operations
fn hash_map_operations(size: usize) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    for i in 0..size {
        let key = format!("key_{}", fibonacci(i as u64 % 10));
        let value = heavy_computation(i as u64 % 500);
        map.insert(key, value);
    }
    map
}

// Vector sorting and manipulation
fn vector_operations(size: usize) -> Vec<u64> {
    let mut vec: Vec<u64> = (0..size as u64)
        .map(|x| fibonacci(x % 15))
        .collect();
    vec.sort_unstable();
    vec.dedup();
    vec
}

// Recursive function with deep call stack
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// CPU-intensive computation
fn heavy_computation(iterations: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 0..iterations {
        result = result.wrapping_add(i.wrapping_mul(i));
    }
    result
}

// Memory allocation intensive function
fn memory_intensive() -> Vec<String> {
    let mut data = Vec::with_capacity(10_000);
    for i in 0..10_000 {
        data.push(format!("Item {}: {}", i, string_processing()));
    }
    data
}

// Complex data processing pipeline
fn process_data_pipeline(size: u64) -> HashMap<String, u64> {
    let data = generate_complex_data(size);
    let processed = transform_complex_data(data);
    aggregate_complex_results(processed)
}

fn generate_complex_data(size: u64) -> Vec<(String, u64)> {
    let mut data = Vec::with_capacity(size as usize);
    for i in 0..size {
        let key = string_processing();
        let value = binary_tree_sum((i % 10) as u32);
        data.push((key, value));
    }
    data
}

fn transform_complex_data(data: Vec<(String, u64)>) -> Vec<(String, u64)> {
    data.into_iter()
        .map(|(k, v)| {
            let new_value = heavy_computation(v % 1000);
            let new_key = format!("processed_{}", k);
            (new_key, new_value)
        })
        .collect()
}

fn aggregate_complex_results(data: Vec<(String, u64)>) -> HashMap<String, u64> {
    let mut result = HashMap::new();
    for (key, value) in data {
        result.insert(key, value);
    }
    result
}

// Async work simulation with complex operations
async fn async_complex_work() {
    for i in 0..3 {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let _ = vector_operations(1000);
        let _ = hash_map_operations(500);
        let _ = process_data_pipeline(100);
        if i % 2 == 0 {
            let _ = binary_tree_sum(10);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let task_type = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "mixed".to_string());

    log::info!("Starting profiling for task type: {}", task_type);
    let guard = ProfilerGuard::new(100).unwrap();

    // Execute the requested task type
    match task_type.as_str() {
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

    // Get and send profile data
    match guard.report().build() {
        Ok(report) => {
            match report.pprof() {
                Ok(profile) => {
                    let mut content = Vec::new();
                    if let Err(e) = profile.encode(&mut content) {
                        log::error!("Failed to encode profile: {}", e);
                        return Err(e.into());
                    }
                    
                    let mut client = MyServiceClient::connect("http://[::1]:50051").await?;
                    let request = Request {
                        data: content,
                    };
                    let response = client.handle_request(request).await?;
                    println!("Profile ID: {}", String::from_utf8_lossy(&response.into_inner().result));
                    Ok(())
                }
                Err(e) => {
                    log::error!("Failed to generate pprof: {}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            log::error!("Failed to build report: {}", e);
            Err(e.into())
        }
    }
} 