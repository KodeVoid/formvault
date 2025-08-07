use actix_web::{App, HttpServer, web};
use routes::health_check;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::TcpListener;
mod handlers;
use dotenvy::dotenv;
use std::env;
mod routes;

/// Asynchronously connects to the Postgres database using `sqlx`
/// Asynchronously connects to the Postgres database using `sqlx`
pub async fn connect_db() -> PgPool {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect("{database_url}")
        .await
        .expect("❌ Failed to connect to Postgres")
}

/// Runs the Actix server with the provided listener and DB connection
pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<actix_web::dev::Server> {
    let db_data = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(routes::health_check::init)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
