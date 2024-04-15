use std::str::FromStr;

use axum::{
    routing::{get, post},
    Router,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use tokio::net::TcpListener;
use tracing::{error, info, level_filters::LevelFilter};

use crate::cli::Cli;

mod api;
mod cli;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let filter = LevelFilter::from_str(&cli.log_level).unwrap_or(LevelFilter::INFO);
    tracing_subscriber::fmt().with_max_level(filter).init();

    if let Err(e) = run(&cli).await {
        error!("{}", e);
        std::process::exit(1);
    }
}

async fn run(cli: &Cli) -> Result<()> {
    let db = edgedb_tokio::create_client()
        .await
        .context("failed to create the edgedb client")?;
    info!("edgedb client connected successfully");

    let app = Router::new()
        .route("/state", get(api::get_state))
        .route("/report", post(api::post_report))
        .with_state(db)
        .fallback(api::not_found);

    let listener = TcpListener::bind(&cli.address)
        .await
        .map_err(|e| anyhow!("failed to bind tcp socket: {}", e))?;
    info!("http server listening on {}", cli.address);

    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow!("failed to start web server: {}", e))?;
    Ok(())
}
