use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PostId(String);

impl PostId {
    pub fn new(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Default for PostId {
    fn default() -> Self {
        Self::new("")
    }
}

impl std::fmt::Display for PostId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PostId {
    fn from(value: String) -> Self {
        PostId(value)
    }
}

impl Into<String> for PostId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: String,
    pub password: String,
}
