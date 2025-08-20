use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PostId(Uuid);

impl PostId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PostId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub password: String,
}
