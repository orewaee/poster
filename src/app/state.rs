use std::{collections::HashMap, sync::Arc};

use crate::post::sqlite::SqlitePostRepository;

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<SqlitePostRepository>,
    pub sessions: Arc<std::sync::Mutex<HashMap<String, Vec<String>>>>,
}

impl AppState {
    pub fn new(post_repository: Arc<SqlitePostRepository>) -> Self {
        Self {
            post_repository,
            sessions: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn load_test_sessions(&self) {
        self.add_session("abcd", "singly-linked-list");
    }

    pub fn has_session(&self, session_id: &str, post_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        dbg!(sessions.clone());
        if let Some(posts) = sessions.get(session_id) {
            posts.contains(&post_id.to_string())
        } else {
            sessions.contains_key(session_id)
        }
    }

    pub fn add_session(&self, session_id: &str, post_id: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions
            .entry(session_id.to_string())
            .or_insert_with(Vec::new)
            .push(post_id.to_string());
    }

    pub fn println(&self) {
        let sessions = self.sessions.lock().unwrap();
        for (k, v) in sessions.iter() {
            println!("{k}: {:?}", v);
        }
    }
}
