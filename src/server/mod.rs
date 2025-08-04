use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

pub mod handlers;
mod routes;

pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::health_check::int))
        .listen(listener)?
        .run()
        .await
}
