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
    let listener = TcpListener::bind("localhost:0").unwrap();
    let port_addr = listener.local_addr().unwrap();

    let formvault = FormVault::new(database_pool, listener);
    println!("Server Running on localhost:{:?}", port_addr);

    formvault.start().await?;
    Ok(port_addr)
}
