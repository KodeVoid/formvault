mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod webhook;
use std::net::SocketAddr;
use std::net::TcpListener;
use models::formvault::FormVault;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn run() -> Result<SocketAddr, std::io::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    
    // Fix: Use PORT environment variable and bind to 0.0.0.0
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&bind_addr).unwrap();
    let port_addr = listener.local_addr().unwrap();
    let formvault = FormVault::new(database_pool, listener);
    
    println!("Server Running on {}", bind_addr);
    formvault.start().await?;
    Ok(port_addr)
}