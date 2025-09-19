use env_logger::Env;
use formvault;
use log::info;
use std::net::SocketAddr;

use formvault::test_helpers::spawn_app;
#[tokio::test] 
async fn test_health_check_point_works() {
    let addr = spawn_app().await;

    let url = format!("http://{}/health_check", addr);
    let response = reqwest::get(&url).await.expect("Failed to send request");

    info!("Response: {:?}", response);

    assert!(response.status().is_success());
}
