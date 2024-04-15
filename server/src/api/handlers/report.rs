use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{models::Reason, utils};

use crate::api::{
    middleware::JsonParser,
    response::{AppError, Status},
};

#[derive(Debug, Deserialize)]
pub struct ReportPayload {
    steam_id: String,
    reason: Reason,
}

#[derive(Debug, Serialize)]
pub struct ReportResponse {
    status: Status,
}

/// Endpoint: /report
/// Method: POST
/// ===============================
/// Stores a new player report in the database.
/// This endpoint is called by any client when a user is reported.
pub async fn post_report(
    headers: HeaderMap,
    State(db): State<edgedb_tokio::Client>,
    JsonParser(payload): JsonParser<ReportPayload>,
) -> Result<Json<ReportResponse>, AppError> {
    if !utils::is_steamid3(&payload.steam_id) {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            anyhow!("the specified steam_id is not supported"),
        ));
    }

    if let Some(hash) = utils::extract_source_hash(&headers) {
        if let Err(err) = db
            .execute(
                // TODO: improve once the support for using Enums as
                //       QueryArgs is added to the Rust driver.
                &format!(
                    r#"with target := (
                        insert User {{
                            steam_id := <str>$0
                        }} unless conflict on .steam_id else User
                    )

                    insert Report {{
                        target := target,
                        author_hash := <str>$1,
                        reason := <default::Reason>'{}',
                    }}"#,
                    payload.reason.to_string(),
                ),
                &(payload.steam_id, hash),
            )
            .await
        {
            warn!("failed to insert report: {}", err);
            Err(AppError(
                StatusCode::BAD_REQUEST,
                anyhow!("the requested operation was not applied"),
            ))
        } else {
            Ok(Json(ReportResponse {
                status: Status::Success,
            }))
        }
    } else {
        warn!("report request does not include the source hash!");
        Err(AppError(
            StatusCode::BAD_REQUEST,
            anyhow!("the request is missing some required fields"),
        ))
    }
}
