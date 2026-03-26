use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

pub struct AppError(pub StatusCode, pub anyhow::Error);

impl AppError {
    pub fn not_found(msg: &str) -> Self {
        AppError(StatusCode::NOT_FOUND, anyhow::anyhow!("{}", msg))
    }

    pub fn conflict(msg: &str) -> Self {
        AppError(StatusCode::CONFLICT, anyhow::anyhow!("{}", msg))
    }

    pub fn unprocessable(msg: &str) -> Self {
        AppError(StatusCode::UNPROCESSABLE_ENTITY, anyhow::anyhow!("{}", msg))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("error: {:?}", self.1);
        (
            self.0,
            Json(json!({ "error": self.1.to_string() }))
        ).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        let error: anyhow::Error = e.into();
        let status = if error.to_string().contains("duplicate key") {
            StatusCode::CONFLICT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        AppError(status, error)
    }
}
