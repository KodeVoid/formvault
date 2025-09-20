use crate::routes;
use actix_web::{App, HttpServer, dev::Server, middleware::Logger};
use log::info;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct FormVault {
    database_pool: PgPool,
    listener: TcpListener,
}
impl FormVault {
    pub fn new(pool: PgPool, listener: TcpListener) -> Self {
        Self {
            database_pool: pool,
            listener,
        }
    }

    pub fn start(self) -> std::io::Result<Server> {
        // Remove async here
        let pool = self.database_pool.clone();
        let addr = self.listener.local_addr().unwrap();
        info!("Starting HTTP server on {}", addr);
        let server = HttpServer::new(move || {
            App::new()
                // enable Actix built-in request logging
                .wrap(Logger::default())
                // make DB pool available to handlers
                .app_data(pool.clone())
                // configure routes
                .configure(routes::configuration::health_check).configure(routes::configuration::api_routes)
        })
        .listen(self.listener)?
        .run();
        Ok(server)
    }
}
