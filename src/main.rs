// Example of using pprof-rs directly
use pprof::ProfilerGuard;
use pprof::protos::Message;
use std::thread;
use std::time::Duration;

mod proto;


#[tokio::main]
async fn main() {
    // Create a guard with higher sampling frequency (1000 Hz)
    let guard = ProfilerGuard::new(1000).unwrap();

    // More substantial work to profile
    for _ in 0..5 {
        let mut sum = 0_u64;
        for i in 0..1_000_000_u64 {
            sum = sum.saturating_add(i);
        }
        thread::sleep(Duration::from_millis(100));
    }

    // Generate report in pprof format
    if let Ok(report) = guard.report().build() {
        if let Ok(profile) = report.pprof() {
            let mut content = Vec::new();
            profile.encode(&mut content).unwrap();
            println!("Profile size: {} bytes", content.len());

            // send pprof data to the server
            let client = PprofClient::connect("http://[::1]:50051").await.unwrap();
            let request = SaveProfileRequest { profile: Some(Profile { data: content }) };
            let _ = client.save_profile(request).await;
        }
    }
}