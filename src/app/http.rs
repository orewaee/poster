use std::{collections::HashMap, fs, sync::Arc};

use askama::Template;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::COOKIE},
    response::{AppendHeaders, Html, IntoResponse},
    routing::{get, post},
};
use comrak::Options;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{
    app::{error::ApiError, params::HttpParams, state::AppState},
    post::{sqlite::SqlitePostRepository, traits::PostRepository},
};

pub async fn run(params: HttpParams) {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("failed to connect to database: {}", e);
            return;
        }
    };

    let post_repository = SqlitePostRepository::new(pool)
        .await
        .expect("failed to create sqlite repository");
    let mut app_state = AppState::new(Arc::new(post_repository));
    // app_state.load_test_sessions();
    let static_service = ServeDir::new(params.static_path);
    let router = Router::new()
        .nest_service("/static", static_service)
        .route("/{id}", get(handle_post))
        .route("/api/login", post(handle_login))
        .with_state(app_state);

    let addr = format!("{}:{}", params.host, params.port);
    println!("running app on {}...", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    id: String,
    content: String,
}

#[derive(Template)]
#[template(path = "password.html")]
struct PasswordTemplate {
    id: String,
}

fn extract_cookie(headers: HeaderMap, name: &str) -> Option<String> {
    if let Some(cookies) = headers.get(COOKIE) {
        let str = cookies.to_str().unwrap_or("");
        if str.is_empty() {
            return None;
        }

        let cookies: HashMap<String, String> = str
            .split(';')
            .map(|pair| pair.trim())
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?.to_string();
                let value = parts.next()?.to_string();
                Some((key, value))
            })
            .collect();

        if let Some(session_id) = cookies.get(name) {
            return Some(session_id.clone());
        } else {
            return None;
        }
    } else {
        return None;
    }
}

#[derive(Debug, Clone, Deserialize)]
struct LoginRequest {
    id: String,
    password: String,
}

async fn handle_login(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    dbg!(request.clone());

    let session_id = if let Some(session_id) = extract_cookie(headers, "session_id") {
        session_id
    } else {
        Uuid::new_v4().to_string()
    };

    match state.post_repository.get_by_id(request.id.clone()).await {
        Ok(post) => {
            if post.password == request.password {
                state.add_session(session_id.as_str(), &post.id);

                (
                    StatusCode::OK,
                    AppendHeaders([("HX-Refresh", "true")]),
                    "Login successful",
                )
                    .into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid password").into_response()
            }
        }
        Err(_error) => (StatusCode::UNAUTHORIZED, "Post not found").into_response(),
    }
}

async fn handle_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<Html<String>, ApiError> {
    let session_id = extract_cookie(headers, "session_id");
    if session_id.is_none() {
        return Err(ApiError::Unauthorized);
    }

    let session_id = session_id.unwrap();

    // TODO: check post.password is None

    if !state.has_session(session_id.as_str(), &id) {
        return Ok(Html(PasswordTemplate { id }.render().unwrap()));
    }

    match state.post_repository.as_ref().get_by_id(id).await {
        Ok(post) => {
            let path = format!("posts/{}.md", post.id);
            dbg!(path.clone());
            dbg!(post.clone());
            return match fs::read_to_string(&path) {
                Ok(content) => {
                    dbg!(content.clone());
                    let content = comrak::markdown_to_html(&content, &Options::default());
                    let template = PostTemplate {
                        id: post.id,
                        content,
                    };

                    Ok(Html(template.render().unwrap()))
                }
                Err(_) => Err(ApiError::PostNotFound),
            };
        }
        Err(error) => Err(ApiError::PostNotFound),
    }
}
