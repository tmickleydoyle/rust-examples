use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::{AppError, Result};
use crate::models::post::{CreatePostRequest, Post, UpdatePostRequest};

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, post: &CreatePostRequest, author_id: Uuid) -> Result<Post> {
        let published = post.published.unwrap_or(false);

        let post = sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (title, content, author_id, published)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, author_id, published, created_at, updated_at
            "#,
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(author_id)
        .bind(published)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(post)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, title, content, author_id, published, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(post)
    }

    pub async fn update(&self, id: Uuid, post: &UpdatePostRequest) -> Result<Option<Post>> {
        // Check if the post exists and get current values
        let existing = self.find_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let existing = existing.unwrap();

        // Update only the fields that are provided
        let title = post.title.clone().unwrap_or(existing.title);
        let content = post.content.clone().unwrap_or(existing.content);
        let published = post.published.unwrap_or(existing.published);

        let updated_post = sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts
            SET title = $1, content = $2, published = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING id, title, content, author_id, published, created_at, updated_at
            "#,
        )
        .bind(&title)
        .bind(&content)
        .bind(published)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(Some(updated_post))
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list(&self, limit: i64, offset: i64, published_only: bool) -> Result<Vec<Post>> {
        let query = if published_only {
            r#"
            SELECT id, title, content, author_id, published, created_at, updated_at
            FROM posts
            WHERE published = true
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        } else {
            r#"
            SELECT id, title, content, author_id, published, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        };

        let posts = sqlx::query_as::<_, Post>(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(posts)
    }

    pub async fn find_by_author(&self, author_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, title, content, author_id, published, created_at, updated_at
            FROM posts
            WHERE author_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(author_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(posts)
    }
}