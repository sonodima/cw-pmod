use anyhow::anyhow;
use axum::http::StatusCode;

use crate::utils::AppError;

pub async fn not_found() -> AppError {
    AppError(
        StatusCode::NOT_FOUND,
        anyhow!("the resource you requested does not exist"),
    )
}
