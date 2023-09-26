use crate::app::{AppState, ExporterState};

use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use prometheus_client::{
    encoding::text::encode as prom_encode, metrics::counter::Counter, registry::Registry,
};

pub fn construct_exporter_state() -> ExporterState {
    let mut registry = Registry::default();

    let under_voltage = Counter::default();
    registry.register(
        "under_voltage",
        "How many the kernel detected under voltage",
        under_voltage.clone(),
    );

    ExporterState {
        registry: Arc::new(registry),
        under_voltage,
    }
}

pub async fn render_metrics(State(state): State<AppState>) -> (StatusCode, String) {
    let mut metrics_buffer = String::new();

    match prom_encode(&mut metrics_buffer, &state.exporter.registry) {
        Ok(()) => (StatusCode::OK, metrics_buffer),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
