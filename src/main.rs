use formvault::server::{connect_db, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    println!("Starting server on {}", listener.local_addr().unwrap());

    let db = connect_db().await;
    let server = run(listener, db).expect("Could not bind server");
    server.await.expect("Server failed");
}
