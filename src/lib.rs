mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod webhook;

use actix_web::dev::Server;
use log::{error, info};
use models::formvault::FormVault;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::{SocketAddr, TcpListener};

pub async fn run() -> Result<(SocketAddr, Server), std::io::Error> {
    info!("Getting DATABASE_URL...");

    let database_url = match env::var("DATABASE_URL") {
        Ok(string) => {
            info!("DATABASE_URL found");
            string
        }
        Err(e) => {
            error!("DATABASE_URL not set in current environment: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DATABASE_URL missing",
            ));
        }
    };

    let database_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Successfully connected to database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                format!("Database connection failed: {e}"),
            ));
        }
    };

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    let listener = match TcpListener::bind(&bind_addr) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Could not bind listener to {}: {}", bind_addr, e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Port binding failed: {e}"),
            ));
        }
    };

    let port_addr = listener.local_addr()?;

    let formvault = FormVault::new(database_pool, listener);

    info!("Server running on {}", bind_addr);
    let server = formvault.start().await?;
    Ok((port_addr, server))
}
