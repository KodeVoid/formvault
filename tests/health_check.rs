use formvault::server::{run, connect_db};
use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let db = connect_db().await;
    let server = run(listener, db).expect("Failed to start server");
    tokio::spawn(server); // runs in background

    address
}

#[tokio::test]
async fn health_check() {
    let address = spawn_app().await; // ✅ fixed here
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
