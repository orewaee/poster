use std::sync::Arc;

use crate::{
    post::sqlite::SqlitePostRepository,
    session::store::{MemorySessionStore, SessionStore},
};

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<SqlitePostRepository>,
    pub session_store: Arc<dyn SessionStore + Send + Sync>,
}

impl AppState {
    pub fn new(post_repository: SqlitePostRepository, session_store: MemorySessionStore) -> Self {
        Self {
            post_repository: Arc::new(post_repository),
            session_store: Arc::new(session_store),
        }
    }
}
