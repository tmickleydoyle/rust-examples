pub mod posts;
pub mod users;

use axum::{routing::get, Json, Router};
use serde_json::json;
use sqlx::PgPool;
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn create_router(pool: PgPool) -> Router {
    // Create a router for API endpoints
    let api_router = Router::new()
        .route("/", get(root_handler))
        .nest("/api/posts", posts::create_router(pool.clone()))
        .nest("/api/users", users::create_router(pool.clone()))
        .route("/health", get(health_check));
    
    // Serve static files from the public directory
    let public_path = PathBuf::from("public");
    
    api_router.nest_service("/ui", ServeDir::new(public_path))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Blog API Server",
        "version": "0.1.0",
        "endpoints": {
            "health": "/health",
            "users": "/api/users",
            "posts": "/api/posts",
            "ui": "/ui"
        },
        "documentation": "See README.md for API documentation"
    }))
}