use std::fs;

use askama::Template;
use axum::{Router, extract::Path, response::Html, routing::get};
use markdown::{Constructs, Options, ParseOptions};
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
#[template(path = "base.html")]
struct PostTemplate {
    content: String,
}

async fn handle_post(Path(id): Path<String>) -> Result<Html<String>, ApiError> {
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
