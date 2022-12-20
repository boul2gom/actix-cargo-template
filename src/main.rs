use actix_web::middleware::{Compress, Logger};
use actix_web::rt::System;
use actix_web::{App, HttpServer, Scope};
use std::env;
use tokio::runtime::Builder;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    log::info!("Starting server with {} worker threads...", worker_threads);

    System::with_tokio_rt(|| {
        Builder::new_multi_thread()
            .thread_name("{{project-name}}-worker")
            .worker_threads(worker_threads)
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
    .block_on(async_bootstrap(worker_threads))
}

async fn async_bootstrap(worker_threads: usize) -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    log::info!("Starting server on {}...", &address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(Scope::new("/api"))
    })
    .workers(worker_threads)
    .bind(address)?
    .run()
    .await
}
