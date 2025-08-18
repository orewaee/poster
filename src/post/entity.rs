#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub password: String,
}
