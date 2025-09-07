use env_logger::Env;
use log::error;

mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod webhook;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    match formvault::run().await {
        Ok((_addr, server)) => {
            if let Err(e) = server.await {
                error!("Server failed while running: {}", e);
            }
        }
        Err(e) => {
            error!("Server failed to start: {}", e);
        }
    }
}
