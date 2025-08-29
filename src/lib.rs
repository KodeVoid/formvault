mod models;
mod handlers;
mod routes;
mod errors;
mod repositories;
mod webhook;
use std::net::SocketAddr;
use actix_web::App;

use actix_web::HttpServer;
use std::net::TcpListener;
use std::error::Error;

use models::formvault::FormVault;


use sqlx::SqlitePool;
use std::env;


pub async fn run()->Result<SocketAddr,std::io::Error>{
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let database_pool = SqlitePool::connect(&database_url).await.unwrap();
	let listener =TcpListener::bind("localhost:0").unwrap();
	let port_addr =listener.local_addr().unwrap();

	let formvault=FormVault::new(database_pool,listener);
		println!("Server Running on localhost:{:?}",port_addr);

	formvault.start().await?;
	Ok(port_addr)
}
