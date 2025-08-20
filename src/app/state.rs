use std::sync::Arc;

use crate::{
    post::store::SqlitePostStore,
    session::store::{MemorySessionStore, SessionStore},
};

#[derive(Clone)]
pub struct AppState {
    pub post_store: Arc<SqlitePostStore>,
    pub session_store: Arc<dyn SessionStore + Send + Sync>,
}

impl AppState {
    pub fn new(post_store: SqlitePostStore, session_store: MemorySessionStore) -> Self {
        Self {
            post_store: Arc::new(post_store),
            session_store: Arc::new(session_store),
        }
    }
}
