use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::db::UserRepository;
use crate::errors::{AppError, Result};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool)
}

#[derive(Debug, Deserialize)]
struct Pagination {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    10
}

async fn list_users(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<UserResponse>>> {
    let repo = UserRepository::new(pool);
    let users = repo.list(pagination.limit, pagination.offset).await?;
    
    let response = users.into_iter().map(UserResponse::from).collect();
    Ok(Json(response))
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    // Validate the request
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // Check if user with email already exists
    let repo = UserRepository::new(pool);
    let existing = repo.find_by_email(&payload.email).await?;
    
    if existing.is_some() {
        return Err(AppError::BadRequest("User with this email already exists".to_string()));
    }
    
    let user = repo.create(&payload).await?;
    Ok(Json(UserResponse::from(user)))
}

async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let repo = UserRepository::new(pool);
    let user = repo.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFoundError(format!("User with id {} not found", id)))?;
    
    Ok(Json(UserResponse::from(user)))
}

async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    // Validate the request if any fields are provided
    if payload.username.is_some() || payload.email.is_some() || payload.password.is_some() {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    }
    
    let repo = UserRepository::new(pool);
    let user = repo.update(id, &payload).await?
        .ok_or_else(|| AppError::NotFoundError(format!("User with id {} not found", id)))?;
    
    Ok(Json(UserResponse::from(user)))
}

async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let repo = UserRepository::new(pool);
    let deleted = repo.delete(id).await?;
    
    if !deleted {
        return Err(AppError::NotFoundError(format!("User with id {} not found", id)));
    }
    
    Ok(Json(serde_json::json!({ "message": "User deleted successfully" })))
}