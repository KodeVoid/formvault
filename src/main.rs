mod handlers;
mod models;
mod routes;
mod repositories;
mod webhook;
mod errors;
#[tokio::main]
async fn main() {
	let _ = formvault::run().await;
}