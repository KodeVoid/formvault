use env_logger::Env;
use std::net::SocketAddr;

use crate::run;
pub async fn spawn_app() -> SocketAddr {
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .is_test(true)
        .try_init();

    let (addr, server) = run().await.expect("Failed to start app");
    tokio::spawn(server);
    addr
}
