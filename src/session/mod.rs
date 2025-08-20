pub mod entity;
pub mod store;

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionData {
    pub authorized_posts: HashSet<String>,
    pub created_at: std::time::Instant,
}

#[derive(Clone)]
pub struct SessionStore {
    pub sessions: Arc<RwLock<HashMap<String, SessionData>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_session(&self) -> String {
        let session_id = Uuid::new_v4().to_string();

        let session_data = SessionData {
            authorized_posts: HashSet::new(),
            created_at: std::time::Instant::now(),
        };

        self.sessions
            .write()
            .unwrap()
            .insert(session_id.clone(), session_data);

        session_id
    }

    pub fn get_session(&self, session_id: &str) -> Option<SessionData> {
        self.sessions.read().unwrap().get(session_id).cloned()
    }
}
