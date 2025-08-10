use axum::{http::StatusCode, response::IntoResponse};

pub enum ApiError {
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
