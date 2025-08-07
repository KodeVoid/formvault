use actix_web::{App, HttpServer, web};
use sea_orm::{Database, DatabaseConnection};
use std::net::TcpListener;

pub mod handlers;
mod routes;

/// Asynchronously connects to the Postgres database
pub async fn connect_db() -> DatabaseConnection {
    Database::connect("postgres://admin:admin@localhost/formvault_db")
        .await
        .expect("Failed to connect to Postgres")
}

/// Runs the Actix server with the provided listener and DB connection
pub fn run(
    listener: TcpListener,
    db: DatabaseConnection,
) -> std::io::Result<actix_web::dev::Server> {
    let db_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) // inject db into all handlers
            .configure(routes::health_check::int)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
