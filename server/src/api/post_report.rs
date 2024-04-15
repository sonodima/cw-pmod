use std::{net::IpAddr, str::FromStr};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};

use anyhow::anyhow;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::warn;

use crate::{
    models::Reason,
    utils::{AppError, JsonParser, Status},
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
    // TODO: add modern text SteamID validation

    if let Some(hash) = extract_source_hash(&headers) {
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

/// Extracts the client's IP Address from CloudFlare's CF-Connecting-IP
/// HTTP header, and generates a BASE64 SHA256 hash with it.
///
/// Of course this makes the assumption that the service is hosted
/// behind CloudFlare. :)
fn extract_source_hash(headers: &HeaderMap) -> Option<String> {
    headers
        .get("CF-Connecting-IP")
        .filter(|ip| IpAddr::from_str(ip.to_str().unwrap_or_default()).is_ok())
        .map(|ip| {
            let mut hasher = Sha256::new();
            hasher.update(ip.as_bytes());
            let hash = hasher.finalize();
            BASE64_STANDARD.encode(&hash)
        })
}
