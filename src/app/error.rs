use axum::{http::StatusCode, response::IntoResponse};

pub enum ApiError {
    PostNotFound,
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::PostNotFound => (StatusCode::NOT_FOUND, "Post not found"),
            Self::Unauthorized => (StatusCode::NOT_FOUND, "Unauthorized"),
        }
        .into_response()
    }
}
