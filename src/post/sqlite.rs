use std::error::Error;

use crate::post::entity::Post;
use crate::post::traits::PostRepository;
use sqlx::SqlitePool;

pub struct SqlitePostRepository {
    pool: SqlitePool,
}

impl SqlitePostRepository {
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

impl PostRepository for SqlitePostRepository {
    type Error = sqlx::Error;

    async fn get_by_id(&self, id: String) -> Result<Post, Self::Error> {
        let (id, password) = sqlx::query_as("SELECT id, password FROM posts WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Post { id, password })
    }

    async fn create(&self, new_post: Post) -> Result<String, Self::Error> {
        let id = new_post.id.clone();

        sqlx::query("INSERT INTO posts (id, password) VALUES (?, ?)")
            .bind(&new_post.id)
            .bind(&new_post.password)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }
}
