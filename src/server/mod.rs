use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub mod handlers;
mod routes;

pub fn run(listener: TcpListener) -> std::io::Result<actix_web::dev::Server> {
    let server = HttpServer::new(|| App::new().configure(routes::health_check::int))
        .listen(listener)?
        .run();

    Ok(server)
}
