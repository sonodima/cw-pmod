use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use anyhow::Error;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Success,
    Error,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

pub struct AppError(pub StatusCode, pub Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response = ErrorResponse {
            status: Status::Error,
            message: self.1.to_string(),
        };

        (self.0, Json(response)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
