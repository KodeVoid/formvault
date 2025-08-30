use env_logger::Env;
use formvault;
use log::info;
use std::net::SocketAddr;

async fn spawn_app() -> SocketAddr {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .is_test(true)
        .try_init();

    let (addr, server) = formvault::run().await.expect("Failed to start app");
    tokio::spawn(server);
    addr
}

#[tokio::test] // instead of #[test]
async fn test_health_check_point_works() {
    let addr = spawn_app().await;

    let url = format!("http://{}/health_check", addr);
    let response = reqwest::get(&url).await.expect("Failed to send request");

    info!("Response: {:?}", response);

    assert!(response.status().is_success());
}
