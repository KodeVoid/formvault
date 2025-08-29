mod handlers;
mod models;
mod routes;
#[tokio::main]
async fn main() {
	let _ = formvault::run().await;
}