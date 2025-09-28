/*!
# FormVault Library Crate

This library exposes the main entry points for running the FormVault
application. It provides:

- [`run`] — starts the application using configuration from environment variables.
- [`spawn_app`] — utility function for starting the server inside integration tests.

## Modules

- `errors` — application error definitions
- `handlers` — request handlers
- `models` — database and domain models
- `repositories` — database repository logic
- `routes` — route configuration
- `webhook` — webhook handling

## Quick Start

```rust,no_run
use formvault;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (_addr, server) = formvault::run().await?;
    server.await
}
```
*/

mod errors;
mod handlers;
mod models;
pub mod repositories;
mod routes;
mod webhook;

use actix_web::dev::Server;
use dotenv::dotenv;
use log::{error, info};
use models::formvault::FormVault;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::{SocketAddr, TcpListener};

/**
Starts the FormVault application.

This function:
1. Loads environment variables from .env (if present).
2. Fetches the `DATABASE_URL` from the environment.
3. Connects to the PostgreSQL database using [`sqlx`].
4. Reads the `PORT` environment variable (default: 8080).
5. Binds a TCP listener on the specified address.
6. Initializes [`FormVault`] with the database pool and TCP listener.
7. Starts the Actix-web server.

# Returns

On success, returns a tuple containing:
- [`SocketAddr`] — the bound network address (IP + port).
- [`Server`] — the running Actix-web server instance.

On failure, returns a [`std::io::Error`] describing the cause.

# Environment Variables

- `DATABASE_URL` — PostgreSQL connection string (required).
- `PORT` — TCP port to bind the server to (default: 0 for random available port).

# Errors

This function will return an error if:
- `DATABASE_URL` environment variable is missing ([`ErrorKind::NotFound`]).
- Database connection fails ([`ErrorKind::Other`]).
- Port binding fails ([`ErrorKind::Other`]).

# Examples

Basic usage:
```rust,no_run
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (addr, server) = formvault::run().await?;
    println!("Server running on {}", addr);
    server.await
}
```

[`ErrorKind::NotFound`]: std::io::ErrorKind::NotFound
[`ErrorKind::Other`]: std::io::ErrorKind::Other
[`FormVault`]: models::formvault::FormVault
*/
pub async fn run() -> Result<(SocketAddr, Server), std::io::Error> {
    // Load .env file if available
    dotenv().ok();

    // Load database URL
    info!("Getting DATABASE_URL...");
    let database_url = env::var("DATABASE_URL").map_err(|e| {
        error!("DATABASE_URL not set in current environment: {}", e);
        std::io::Error::new(std::io::ErrorKind::NotFound, "DATABASE_URL missing")
    })?;

    // Initialize database pool
    let database_pool = PgPoolOptions::new()
        .max_connections(5) // Consider making this configurable
        .connect(&database_url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Database connection failed: {e}"),
            )
        })?;

    info!("Successfully connected to database");

    // Read bind address - use 0 for random port if none specified
    let port = env::var("PORT").unwrap_or_else(|_| "0".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    // Bind TCP listener
    let listener = TcpListener::bind(&bind_addr).map_err(|e| {
        error!("Could not bind listener to {}: {}", bind_addr, e);
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Port binding failed: {e}"),
        )
    })?;

    let port_addr = listener.local_addr()?;

    // Initialize and start server
    let formvault = FormVault::new(database_pool, listener);
    let server = formvault.start()?;
    info!("Server successfully started on {} (actual port: {})", bind_addr, port_addr.port());

    Ok((port_addr, server))
}

/// Spawns the FormVault server for use in integration tests.
///
/// This helper function:
/// - Initializes the logger in test mode (quiet output).
/// - Calls [`run`] to start the application.
/// - Spawns the Actix server in a background Tokio task.
/// - Returns the bound [`SocketAddr`] so tests can make HTTP requests.
///
/// # Returns
///
/// The [`SocketAddr`] that the test server is bound to.
///
/// # Panics
///
/// Panics if the application fails to start.
///
/// # Examples
///
/// ```rust,no_run
/// #[tokio::test]
/// async fn test_health_endpoint() {
///     let addr = formvault::spawn_app().await;
///     let url = format!("http://{}/health", addr);
///     
///     let response = reqwest::get(&url).await.unwrap();
///     assert!(response.status().is_success());
/// }
/// ```
pub async fn spawn_app() -> SocketAddr {
    use env_logger::Env;

    // Initialize logging for test context
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .is_test(true)
        .try_init();

    // Start application
    let (addr, server) = run().await.expect("Failed to start app");

    // Run server in background
    tokio::spawn(server);

    addr
}