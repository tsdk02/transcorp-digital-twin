#![allow(dead_code)]
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env if present
    match dotenvy::dotenv() {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(e) => println!("No .env loaded: {}", e),
    }
    env_logger::init();

    let app_config = config::AppConfig::from_env();
    log::info!("Valid auth tokens: {:?}", app_config.valid_auth_tokens);
    log::info!("Valid tenants: {:?}", app_config.valid_tenants);
    let port = app_config.port;

    let app_state = web::Data::new(state::AppState::new(app_config.partner_webhook_url));

    let valid_tokens = Arc::new(app_config.valid_auth_tokens);
    let valid_tenants = Arc::new(app_config.valid_tenants);

    // Clone for shutdown hook
    let shutdown_state = app_state.clone();

    log::info!("Starting Transcorp Digital Twin on port {}", port);
    log::info!("State will be saved to data/state.json on shutdown (Ctrl+C)");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::auth::AuthValidator {
                valid_tokens: valid_tokens.clone(),
                valid_tenants: valid_tenants.clone(),
            })
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", port))?
    .run();

    let server_handle = server.handle();

    // Spawn a task to handle Ctrl+C gracefully
    let shutdown_handle = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Shutting down — saving state...");
        shutdown_state.save_snapshot();
        server_handle.stop(true).await;
    });

    server.await?;
    shutdown_handle.await.ok();

    Ok(())
}
