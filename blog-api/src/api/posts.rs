use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::db::{PostRepository, UserRepository};
use crate::errors::{AppError, Result};
use crate::models::post::{CreatePostRequest, PostResponse, UpdatePostRequest};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_posts).post(create_post))
        .route("/:id", get(get_post).put(update_post).delete(delete_post))
        .route("/user/:user_id", get(list_posts_by_user))
        .with_state(pool)
}

#[derive(Debug, Deserialize)]
struct Pagination {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
    #[serde(default)]
    published_only: bool,
}

fn default_limit() -> i64 {
    10
}

async fn list_posts(
    State(pool): State<PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<PostResponse>>> {
    let repo = PostRepository::new(pool);
    let posts = repo.list(pagination.limit, pagination.offset, pagination.published_only).await?;
    
    let response = posts.into_iter().map(PostResponse::from).collect();
    Ok(Json(response))
}

async fn list_posts_by_user(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<PostResponse>>> {
    // First check if the user exists
    let user_repo = UserRepository::new(pool.clone());
    if user_repo.find_by_id(user_id).await?.is_none() {
        return Err(AppError::NotFoundError(format!("User with id {} not found", user_id)));
    }
    
    let post_repo = PostRepository::new(pool);
    let posts = post_repo.find_by_author(user_id, pagination.limit, pagination.offset).await?;
    
    let response = posts.into_iter().map(PostResponse::from).collect();
    Ok(Json(response))
}

async fn create_post(
    State(pool): State<PgPool>,
    // In a real app, you would get the user_id from the authenticated session
    // For simplicity, we'll use a header or query param
    Query(params): Query<AuthorParam>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>> {
    // Validate the request
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    // Check if the author exists
    let user_repo = UserRepository::new(pool.clone());
    if user_repo.find_by_id(params.author_id).await?.is_none() {
        return Err(AppError::BadRequest(format!("User with id {} does not exist", params.author_id)));
    }
    
    let post_repo = PostRepository::new(pool);
    let post = post_repo.create(&payload, params.author_id).await?;
    
    Ok(Json(PostResponse::from(post)))
}

#[derive(Debug, Deserialize)]
struct AuthorParam {
    author_id: Uuid,
}

async fn get_post(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<PostResponse>> {
    let repo = PostRepository::new(pool);
    let post = repo.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFoundError(format!("Post with id {} not found", id)))?;
    
    Ok(Json(PostResponse::from(post)))
}

async fn update_post(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    // In a real app, you would verify that the user is the author of the post
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>> {
    // Validate the request if any fields are provided
    if payload.title.is_some() || payload.content.is_some() {
        payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    }
    
    let repo = PostRepository::new(pool);
    
    // Check if post exists
    let _post = repo.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFoundError(format!("Post with id {} not found", id)))?;
    
    // In a real app, we would check if the current user is the author
    
    let updated_post = repo.update(id, &payload).await?
        .ok_or_else(|| AppError::NotFoundError(format!("Post with id {} not found", id)))?;
    
    Ok(Json(PostResponse::from(updated_post)))
}

async fn delete_post(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    // In a real app, you would verify that the user is the author of the post
) -> Result<Json<serde_json::Value>> {
    let repo = PostRepository::new(pool);
    
    // Check if post exists
    let _post = repo.find_by_id(id).await?
        .ok_or_else(|| AppError::NotFoundError(format!("Post with id {} not found", id)))?;
    
    // In a real app, we would check if the current user is the author
    
    let deleted = repo.delete(id).await?;
    
    if !deleted {
        return Err(AppError::NotFoundError(format!("Post with id {} not found", id)));
    }
    
    Ok(Json(serde_json::json!({ "message": "Post deleted successfully" })))
}