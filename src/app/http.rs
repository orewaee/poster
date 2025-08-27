use std::{collections::HashMap, fs};

use askama::Template;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::COOKIE},
    response::{AppendHeaders, Html, IntoResponse, Response},
    routing::{get, post},
};
use comrak::{ExtensionOptions, Options, RenderOptions};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;

use crate::{
    app::{error::ApiError, params::HttpParams, state::AppState},
    post::store::{PostStore, SqlitePostStore},
    session::{entity::SessionId, store::MemorySessionStore},
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

    let post_store = SqlitePostStore::new(pool)
        .await
        .expect("failed to create sqlite repository");
    let session_store = MemorySessionStore::new();
    let app_state = AppState::new(post_store, session_store);

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
    with_password: bool,
}

#[derive(Template)]
#[template(path = "password.html")]
struct PasswordTemplate {
    id: String,
}

#[derive(Template)]
#[template(path = "not-found.html")]
struct NotFoundTemplate;

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
) -> Response {
    let session_id: Option<SessionId> =
        if let Some(session_id) = extract_cookie(headers, "session_id") {
            Some(session_id.into())
        } else {
            None
        };

    match state.post_store.get_by_id(request.id.clone().into()).await {
        Ok(post) => {
            if post
                .password
                .is_some_and(|password| password == request.password)
            {
                match state.session_store.create(session_id, post.id.into()) {
                    Ok(session_id) => (
                        StatusCode::OK,
                        AppendHeaders([
                            ("hx-refresh", "true"),
                            (
                                "set-cookie",
                                format!(
                                    "session_id={}; HttpOnly; Secure; Path=/",
                                    session_id.to_string()
                                )
                                .as_str(),
                            ),
                        ]),
                        "Login successful",
                    )
                        .into_response(),
                    Err(error) => (
                        StatusCode::UNAUTHORIZED,
                        format!("failed to create session: {}", error),
                    )
                        .into_response(),
                }
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
    return match state.post_store.as_ref().get_by_id(id.clone().into()).await {
        Ok(post) => {
            if post.password.is_none() {
                let path = format!("posts/{}.md", post.id);
                return match fs::read_to_string(&path) {
                    Ok(content) => {
                        let content = comrak::markdown_to_html(
                            &content,
                            &Options {
                                extension: ExtensionOptions {
                                    table: true,
                                    autolink: true,
                                    header_ids: Some(String::new()),
                                    wikilinks_title_after_pipe: true,
                                    spoiler: true,
                                    ..Default::default()
                                },
                                render: RenderOptions {
                                    gfm_quirks: true,
                                    tasklist_classes: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        );
                        let template = PostTemplate {
                            id: post.id.into(),
                            content,
                            with_password: true,
                        };

                        Ok(Html(template.render().unwrap()))
                    }
                    Err(_) => Err(ApiError::PostNotFound),
                };
            } else {
                let session_id: Option<SessionId> =
                    if let Some(session_id) = extract_cookie(headers, "session_id") {
                        Some(session_id.into())
                    } else {
                        None
                    };

                if session_id.is_none()
                    || !state
                        .session_store
                        .authorized(session_id.unwrap(), id.clone().into())
                        .unwrap()
                {
                    return Ok(Html(PasswordTemplate { id }.render().unwrap()));
                }

                let path = format!("posts/{}.md", post.id);
                return match fs::read_to_string(&path) {
                    Ok(content) => {
                        let content = comrak::markdown_to_html(&content, &Options::default());
                        let template = PostTemplate {
                            id: post.id.into(),
                            content,
                            with_password: false,
                        };

                        Ok(Html(template.render().unwrap()))
                    }
                    Err(_) => Err(ApiError::PostNotFound),
                };
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            Ok(Html(NotFoundTemplate.render().unwrap()))
        }
    };
}
