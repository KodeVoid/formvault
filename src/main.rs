mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod webhook;
#[tokio::main]
async fn main() {
    let _ = formvault::run().await;
}
