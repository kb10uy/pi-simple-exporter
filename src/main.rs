mod app;
mod index;
mod metrics;
mod watcher;

use anyhow::Result;
use axum::{routing::get, Router, Server};
use clap::Parser;
use tokio::spawn;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = app::Arguments::parse();

    // construct state
    let state = app::AppState {
        exporter: metrics::construct_exporter_state(),
    };

    // run watchers
    spawn(watcher::watch_under_voltage(state.clone()));

    // run web server
    let router = Router::new()
        .route("/", get(index::render_index))
        .route("/metrics", get(metrics::render_metrics))
        .with_state(state);

    Server::bind(&args.bind)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
