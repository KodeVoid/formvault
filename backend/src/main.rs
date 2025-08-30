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

    if let Err(e) = formvault::run().await {
        error!("Server failed: {}", e);
    }
}
