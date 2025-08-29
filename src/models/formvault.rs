use actix_web::{App, HttpServer};
use sqlx::SqlitePool;
use std::net::TcpListener;
use crate::routes;
pub struct FormVault {
    database_pool: SqlitePool,
    listener: TcpListener,

}

impl FormVault {
    pub fn new(pool: SqlitePool, listener: TcpListener) -> Self {
        Self { database_pool: pool, listener }
    }

    pub async fn start(self) -> std::io::Result<()> {
        let pool = self.database_pool.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(pool.clone()).configure(routes::configuration::health_check)
        })
        .listen(self.listener)?
        .run()
        .await
    }
}
