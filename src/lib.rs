pub mod myservice {
    tonic::include_proto!("myservice");
}

pub mod tasks {
    use std::collections::HashMap;

    // Recursive tree-like computation
    pub fn binary_tree_sum(depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let left = binary_tree_sum(depth - 1);
        let right = binary_tree_sum(depth - 1);
        left + right + depth as u64
    }

    // String manipulation intensive
    pub fn string_processing() -> String {
        let mut result = String::with_capacity(1000);
        for i in 0..100 {
            result.push_str(&format!("Processing item {}: ", i));
            result.push_str(&fibonacci(i % 15).to_string());
            result.push('\n');
        }
        result
    }

    // Hash map operations
    pub fn hash_map_operations(size: usize) -> HashMap<String, u64> {
        let mut map = HashMap::new();
        for i in 0..size {
            let key = format!("key_{}", fibonacci(i as u64 % 10));
            let value = heavy_computation(i as u64 % 500);
            map.insert(key, value);
        }
        map
    }

    // Vector sorting and manipulation
    pub fn vector_operations(size: usize) -> Vec<u64> {
        let mut vec: Vec<u64> = (0..size as u64)
            .map(|x| fibonacci(x % 15))
            .collect();
        vec.sort_unstable();
        vec.dedup();
        vec
    }

    // Recursive function with deep call stack
    pub fn fibonacci(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        fibonacci(n - 1) + fibonacci(n - 2)
    }

    // CPU-intensive computation
    pub fn heavy_computation(iterations: u64) -> u64 {
        let mut result: u64 = 0;
        for i in 0..iterations {
            result = result.wrapping_add(i.wrapping_mul(i));
        }
        result
    }

    // Memory allocation intensive function
    pub fn memory_intensive() -> Vec<String> {
        let mut data = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            data.push(format!("Item {}: {}", i, string_processing()));
        }
        data
    }

    // Complex data processing pipeline
    pub fn process_data_pipeline(size: u64) -> HashMap<String, u64> {
        let data = generate_complex_data(size);
        let processed = transform_complex_data(data);
        aggregate_complex_results(processed)
    }

    pub fn generate_complex_data(size: u64) -> Vec<(String, u64)> {
        let mut data = Vec::with_capacity(size as usize);
        for i in 0..size {
            let key = string_processing();
            let value = binary_tree_sum((i % 10) as u32);
            data.push((key, value));
        }
        data
    }

    pub fn transform_complex_data(data: Vec<(String, u64)>) -> Vec<(String, u64)> {
        data.into_iter()
            .map(|(k, v)| {
                let new_value = heavy_computation(v % 1000);
                let new_key = format!("processed_{}", k);
                (new_key, new_value)
            })
            .collect()
    }

    pub fn aggregate_complex_results(data: Vec<(String, u64)>) -> HashMap<String, u64> {
        let mut result = HashMap::new();
        for (key, value) in data {
            result.insert(key, value);
        }
        result
    }
}

/// Storage utilities for managing profile data
pub mod storage {
    use std::fs;
    use std::path::PathBuf;
    use std::io;

    /// Initialize the data directory for storing profiles
    /// 
    /// Creates the base directory if it doesn't exist
    pub fn init_data_dir() -> io::Result<()> {
        fs::create_dir_all("data")?;
        Ok(())
    }

    /// Create a new directory for a specific profile
    /// 
    /// # Arguments
    /// * `profile_id` - Unique identifier for the profile
    /// 
    /// # Returns
    /// * `PathBuf` - Path to the created directory
    pub fn create_profile_dir(profile_id: &str) -> io::Result<PathBuf> {
        let dir_path = PathBuf::from("data").join(profile_id);
        fs::create_dir_all(&dir_path)?;
        Ok(dir_path)
    }

    /// Get the path for a profile file
    /// 
    /// # Arguments
    /// * `profile_id` - Unique identifier for the profile
    /// * `extension` - File extension (e.g., "pb" or "json")
    /// 
    /// # Returns
    /// * `PathBuf` - Full path to the profile file
    pub fn get_profile_path(profile_id: &str, extension: &str) -> PathBuf {
        PathBuf::from("data")
            .join(profile_id)
            .join(format!("profile.{}", extension))
    }
} 