use std::fs;

use askama::Template;
use axum::{Router, extract::Path, response::Html, routing::get};
use comrak::Options;
use tower_http::services::ServeDir;

use crate::app::{error::ApiError, params::HttpParams};

pub async fn run(params: HttpParams) {
    let static_service = ServeDir::new(params.static_path);

    let router = Router::new()
        .nest_service("/static", static_service)
        .route("/{id}", get(handle_post));

    let addr = format!("{}:{}", params.host, params.port);
    println!("running app on {}...", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate {
    id: String,
    content: String,
}

async fn handle_post(Path(id): Path<String>) -> Result<Html<String>, ApiError> {
    let path = format!("posts/{}.md", id);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let content = comrak::markdown_to_html(&content, &Options::default());

            // let content = markdown::to_html_with_options(&content, options).unwrap();
            let template = PostTemplate { id, content };
            Ok(Html(template.render().unwrap()))
        }
        Err(_) => Err(ApiError::PostNotFound),
    }
}
