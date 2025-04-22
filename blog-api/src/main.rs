mod api;
mod config;
mod db;
mod errors;
mod models;

use std::net::SocketAddr;

use axum::http::{HeaderValue, Method};
use config::AppConfig;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting blog API server");

    // Load configuration
    let config = AppConfig::from_env().unwrap_or_else(|_| {
        tracing::warn!("Failed to load config from environment, using default development config");
        AppConfig::default_development()
    });

    tracing::info!(
        "Loaded configuration: server at {}:{}, database URL: {}",
        config.server.host, 
        config.server.port,
        // Masked URL to avoid leaking credentials in logs
        config.database.url.split('@').last().unwrap_or("<masked>")
    );

    // Set up database connection
    let pool = db::create_pool(&config.database).await?;
    
    // Run migrations
    tracing::info!("Running database migrations");
    match db::run_migrations(&pool).await {
        Ok(_) => tracing::info!("Migrations completed successfully"),
        Err(e) => {
            tracing::error!("Failed to run migrations: {}", e);
            return Err(anyhow::anyhow!("Failed to run migrations"));
        }
    }

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any); // Allow any origin for browser access

    // Build our application with routes
    let app = api::create_router(pool.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Run the server
    let addr = format!("{}{}{}", config.server.host, ":", config.server.port);
    let socket_addr: SocketAddr = addr.parse()?;
    
    tracing::info!("Listening on {}", socket_addr);
    tracing::info!("Web UI available at: http://{}:{}/ui", config.server.host, config.server.port);
    
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}