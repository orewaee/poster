use sqlx::SqlitePool;
use thiserror::Error;

use crate::post::entity::{Post, PostId};

#[derive(Debug, Error)]
pub enum PostStoreError {
    #[error("post not found")]
    PostNotFound,

    #[error("failed to create post: {0}")]
    FailedToCreatePost(String),

    #[error("unexpected error")]
    Unexpected,
}

pub trait PostStore {
    async fn create(
        &self,
        id: Option<PostId>,
        password: Option<String>,
    ) -> Result<PostId, PostStoreError>;
    async fn get_by_id(&self, id: PostId) -> Result<Post, PostStoreError>;
    async fn delete_by_id(&self, id: PostId) -> Result<bool, PostStoreError>;
}

pub struct SqlitePostStore {
    pool: SqlitePool,
}

impl SqlitePostStore {
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS posts (
            id TEXT PRIMARY KEY,
            password TEXT NOT NULL
        )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

impl PostStore for SqlitePostStore {
    async fn create(
        &self,
        id: Option<PostId>,
        password: Option<String>,
    ) -> Result<PostId, PostStoreError> {
        let id = id.unwrap_or_default();
        if let Some(error) = sqlx::query("INSERT INTO posts (id, password) VALUES ($1, $2)")
            .bind(id.clone())
            .bind(password)
            .execute(&self.pool)
            .await
            .err()
        {
            eprintln!("{}", error);
            Err(PostStoreError::FailedToCreatePost(error.to_string()))
        } else {
            Ok(id)
        }
    }

    async fn get_by_id(&self, id: PostId) -> Result<Post, PostStoreError> {
        let post: Option<Post> = sqlx::query_as("SELECT id, password FROM posts WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        if let Some(post) = post {
            Ok(post)
        } else {
            Err(PostStoreError::PostNotFound)
        }
    }

    async fn delete_by_id(&self, id: PostId) -> Result<bool, PostStoreError> {
        if let Some(error) = sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id.clone())
            .execute(&self.pool)
            .await
            .err()
        {
            eprintln!("{}", error);
            Err(PostStoreError::FailedToCreatePost(error.to_string()))
        } else {
            Ok(true)
        }
    }
}
