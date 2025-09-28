use criterion::{Criterion, criterion_group, criterion_main};
use formvault::{run, spawn_app};
use std::sync::atomic::{AtomicU16, Ordering};

// Static counter for unique ports
static PORT_COUNTER: AtomicU16 = AtomicU16::new(9000);

fn get_unique_port() -> u16 {
    PORT_COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn run_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("run_full_startup", |b| {
        b.iter_with_large_drop(|| {
            rt.block_on(async {
                // Use port 0 for random available port to avoid conflicts
                unsafe {
                    std::env::set_var("PORT", "0");
                }

                match run().await {
                    Ok((addr, _server)) => {
                        // Server will be dropped automatically, stopping it
                        Some(addr)
                    }
                    Err(_) => None,
                }
            })
        })
    });
}

fn spawn_app_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("spawn_app_startup", |b| {
        b.iter_with_large_drop(|| {
            rt.block_on(async {
                // Use port 0 for random available port
                unsafe {
                    std::env::set_var("PORT", "0");
                }

                // spawn_app returns just the address, server runs in background
                let addr = spawn_app().await;

                // Give a moment for the server to fully start
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

                addr
            })
        })
    });
}

fn database_connection_benchmark(c: &mut Criterion) {
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Only run if DATABASE_URL is available
    if let Ok(database_url) = env::var("DATABASE_URL") {
        c.bench_function("database_connection", |b| {
            b.iter(|| {
                rt.block_on(async {
                    let _pool = PgPoolOptions::new()
                        .max_connections(1)
                        .connect(&database_url)
                        .await;
                })
            })
        });
    }
}

fn env_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("env_parsing", |b| {
        b.iter(|| std::env::var("PORT").unwrap_or_else(|_| "0".to_string()))
    });
}

fn address_formatting_benchmark(c: &mut Criterion) {
    c.bench_function("address_formatting", |b| {
        b.iter(|| {
            let port = get_unique_port();
            format!("0.0.0.0:{}", port)
        })
    });
}

fn tcp_listener_benchmark(c: &mut Criterion) {
    use std::net::TcpListener;

    c.bench_function("tcp_listener_bind", |b| {
        b.iter(|| {
            // Use port 0 for random available port
            let addr = "127.0.0.1:0";
            // Just benchmark the binding, then immediately drop
            let _listener = TcpListener::bind(addr);
        })
    });
}

// Benchmark just the setup parts without actually starting the server
fn formvault_config_benchmark(c: &mut Criterion) {
    use dotenv::dotenv;
    use std::env;

    c.bench_function("config_loading", |b| {
        b.iter(|| {
            dotenv().ok();
            let _port = env::var("PORT").unwrap_or_else(|_| "0".to_string());
            let port_num = get_unique_port();
            let _bind_addr = format!("0.0.0.0:{}", port_num);
        })
    });
}

// Alternative benchmark without server startup (faster, more focused)
fn run_config_only_benchmark(c: &mut Criterion) {
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::net::TcpListener;

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Only run if DATABASE_URL is available
    if let Ok(database_url) = env::var("DATABASE_URL") {
        c.bench_function("run_config_setup", |b| {
            b.iter(|| {
                rt.block_on(async {
                    // Benchmark just the configuration parts without server startup
                    dotenv().ok();
                    let _port = env::var("PORT").unwrap_or_else(|_| "0".to_string());
                    let bind_addr = "127.0.0.1:0";
                    let _listener = TcpListener::bind(bind_addr);
                    let _pool = PgPoolOptions::new()
                        .max_connections(1)
                        .connect(&database_url)
                        .await;
                })
            })
        });
    }
}

criterion_group!(
    benches,
    run_benchmark,
    spawn_app_benchmark,
    database_connection_benchmark,
    env_parsing_benchmark,
    address_formatting_benchmark,
    tcp_listener_benchmark,
    formvault_config_benchmark,
    run_config_only_benchmark
);
criterion_main!(benches);
