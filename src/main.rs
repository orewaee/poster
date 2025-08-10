use std::fs;

use askama::Template;
use axum::{
    Router,
    extract::Path,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use markdown::{Constructs, Options, ParseOptions};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/{id}", get(post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2201").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

enum ApiError {
    PostNotFound,
    StylesNotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::PostNotFound => (StatusCode::NOT_FOUND, "Post not found"),
            Self::StylesNotFound => (StatusCode::NOT_FOUND, "Styles not found"),
        }
        .into_response()
    }
}

#[derive(Template)]
#[template(path = "base.html")]
struct PostTemplate {
    content: String,
}

async fn styles() -> Result<Response, ApiError> {
    match fs::read_to_string("static/styles.css") {
        Ok(content) => Ok(([(header::CONTENT_TYPE, "text/css")], content).into_response()),
        Err(_) => Err(ApiError::StylesNotFound),
    }
}

async fn post(Path(id): Path<String>) -> Result<Html<String>, ApiError> {
    dbg!(id.clone());
    let path = format!("posts/{}.md", id);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let options = &markdown::Options {
                parse: ParseOptions {
                    constructs: Constructs {
                        code_text: true,
                        math_text: true,
                        math_flow: true,
                        heading_atx: true,
                        label_start_image: true,
                        ..Constructs::default()
                    },
                    ..ParseOptions::default()
                },
                ..Options::default()
            };

            let content = markdown::to_html_with_options(&content, options).unwrap();
            let hello = PostTemplate { content };
            Ok(Html(hello.render().unwrap()))
        }
        Err(_) => Err(ApiError::PostNotFound),
    }
}
