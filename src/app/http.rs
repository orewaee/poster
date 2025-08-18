use std::{fs, sync::Arc};

use askama::Template;
use axum::{
    Router,
    extract::{Path, State},
    response::Html,
    routing::get,
};
use comrak::Options;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::services::ServeDir;

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
    let app_state = AppState::new(Arc::new(post_repository));
    let static_service = ServeDir::new(params.static_path);
    let router = Router::new()
        .nest_service("/static", static_service)
        .route("/{id}", get(handle_post))
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

async fn handle_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Html<String>, ApiError> {
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
