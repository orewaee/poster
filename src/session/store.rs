use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use thiserror::Error;

use crate::post::entity::PostId;
use crate::session::entity::{SessionData, SessionId};

#[derive(Debug, Error)]
pub enum SessionStoreError {
    #[error("lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("unexpected error")]
    Unexpected,
}

pub trait SessionStore {
    fn create(
        &self,
        session_id: Option<SessionId>,
        post_id: PostId,
    ) -> Result<SessionId, SessionStoreError>;

    fn authorized(&self, session_id: SessionId, post_id: PostId)
    -> Result<bool, SessionStoreError>;
}

pub struct MemorySessionStore {
    sessions: Arc<Mutex<HashMap<SessionId, SessionData>>>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::from(Mutex::new(HashMap::new())),
        }
    }
}

impl SessionStore for MemorySessionStore {
    fn create(
        &self,
        session_id: Option<SessionId>,
        post_id: PostId,
    ) -> Result<SessionId, SessionStoreError> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|error| SessionStoreError::LockPoisoned(error.to_string()))?;

        let session_id = session_id.unwrap_or_default();

        if let Some(existing) = sessions.get_mut(&session_id) {
            existing.authorized_posts.insert(post_id);
            return Ok(session_id);
        }

        let mut session_data = SessionData::new();
        session_data.authorized_posts.insert(post_id);
        sessions.insert(session_id.clone(), session_data);
        Ok(session_id)
    }

    fn authorized(
        &self,
        session_id: SessionId,
        post_id: PostId,
    ) -> Result<bool, SessionStoreError> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|error| SessionStoreError::LockPoisoned(error.to_string()))?;

        Ok(if let Some(session_data) = sessions.get(&session_id) {
            session_data.authorized_posts.contains(&post_id)
        } else {
            false
        })
    }
}
