use std::sync::Arc;

use crate::post::sqlite::SqlitePostRepository;

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<SqlitePostRepository>,
}

impl AppState {
    pub fn new(post_repository: Arc<SqlitePostRepository>) -> Self {
        Self { post_repository }
    }
}
