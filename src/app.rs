use std::{
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
};

use clap::Parser;
use prometheus_client::{metrics::counter::Counter, registry::Registry};

/// pi-simple-exporter
#[derive(Debug, Clone, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// socket address to bind the web server.
    #[clap(short, long, default_value = "0.0.0.0:9100")]
    pub bind: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub exporter: ExporterState,
}

#[derive(Debug, Clone)]
pub struct ExporterState {
    pub registry: Arc<Registry>,
    pub under_voltage: Counter<u64, AtomicU64>,
}
