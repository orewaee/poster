use std::collections::HashSet;
use std::time::Instant;

use uuid::Uuid;

use crate::post::entity::PostId;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SessionId {
    fn from(value: String) -> Self {
        SessionId(Uuid::parse_str(&value).unwrap())
    }
}

impl Into<String> for SessionId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct SessionData {
    pub authorized_posts: HashSet<PostId>,
    pub created_at: Instant,
}

impl SessionData {
    pub fn new() -> Self {
        Self {
            authorized_posts: HashSet::new(),
            created_at: Instant::now(),
        }
    }
}
