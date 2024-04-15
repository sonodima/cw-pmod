use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};

use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    models,
    utils::{AppError, QueryParser, Status},
};

#[derive(Debug, Deserialize)]
pub struct GetStateParams {
    steam_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetStateResponse {
    status: Status,
    should_be_kicked: bool,
    total_reports: usize,
}

/// Endpoint: /state?steam_id=xxxx
/// Method: GET
/// ===============================
/// Queries the informations for a user from the database.
/// This endpoint is called by the host whenever a new player
/// joins the lobby.
pub async fn get_state(
    State(db): State<edgedb_tokio::Client>,
    QueryParser(params): QueryParser<GetStateParams>,
) -> Result<Json<GetStateResponse>, AppError> {
    // TODO: add modern text SteamID validation

    // Get all the reports registered for the provided user's steam_id.
    let result = db
        .query::<models::Report, _>(
            r#"select Report {
                reason
            } filter Report.target.steam_id = <str>$0"#,
            &(params.steam_id,),
        )
        .await;

    if let Ok(reports) = result {
        // Calculate a score, based on the type of reports the user has.
        // TODO: down the road we may want to implement a way to give
        //       cooldowns instead of permabans in some scenarios. (mainly toxicity?)
        // TODO: we could also make it so that it weights reports more when they
        //       all happen in a short timespan?
        Ok(Json(GetStateResponse {
            status: Status::Success,
            should_be_kicked: calc_bad_rep_score(&reports) > 10,
            total_reports: reports.len(),
        }))
    } else {
        error!("failed to query player reports: {}", result.unwrap_err());
        Err(AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            anyhow!("failed to obtain the player's state"),
        ))
    }
}

fn calc_bad_rep_score(reports: &[models::Report]) -> usize {
    reports.into_iter().map(|r| r.reason.weight()).sum()
}
