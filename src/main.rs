use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tracing::info;

mod api;
mod config;
mod domain;
mod error;
mod infrastructure;
mod service;
mod validation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = config::load_config().expect("Failed to load configuration");
    let app_state = web::Data::new(infrastructure::AppState::new(&config).await);

    info!("Starting ISO 20022 Payment Processing Service");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(infrastructure::middleware::request_tracing::RequestTracing)
            .wrap(infrastructure::middleware::error_handling::ErrorHandling)
            .configure(api::payment::config)
    })
    .bind(("127.0.0.1", config.server.port))?
    .run()
    .await
}
