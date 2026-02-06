use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use tokio::sync::RwLock;
use warp::{Filter, Rejection, Reply};

use super::toml::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_messages: u64,
    pub messages_per_minute: u64,
    pub last_message_time: Option<DateTime<Utc>>,
    pub uptime_seconds: u64,
    pub start_time: DateTime<Utc>,
    pub memory_usage_mb: f64,
    pub memory_peak_mb: f64,
    pub last_minute_messages: u64,
    pub last_minute_start: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

static STATS: once_cell::sync::Lazy<Arc<RwLock<Stats>>> = once_cell::sync::Lazy::new(|| {
    let now = Utc::now();
    let stats = Stats {
        total_messages: 0,
        messages_per_minute: 0,
        last_message_time: None,
        uptime_seconds: 0,
        start_time: now,
        memory_usage_mb: 0.0,
        memory_peak_mb: 0.0,
        last_minute_messages: 0,
        last_minute_start: now,
    };
    Arc::new(RwLock::new(stats))
});

fn get_memory_usage(sys: &mut System, current_peak: f64) -> (f64, f64) {
    let pid = match sysinfo::get_current_pid() {
        Ok(pid) => pid,
        Err(_) => return (0.0, current_peak),
    };
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    if let Some(process) = sys.process(pid) {
        let mem_mb = process.memory() as f64 / 1024.0 / 1024.0;
        let peak_mb = current_peak.max(mem_mb);
        return (mem_mb, peak_mb);
    }
    (0.0, current_peak)
}

pub async fn start(config: Arc<Config>) -> Result<()> {
    let port = config.dashboard_port;

    // Start stats updater
    let mut sys = System::new();
    let stats = STATS.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut stats_guard = stats.write().await;
            let now = Utc::now();
            stats_guard.uptime_seconds = (now - stats_guard.start_time).num_seconds() as u64;

            // Check if we need to reset the per-minute counter
            let seconds_since_last_reset = (now - stats_guard.last_minute_start).num_seconds();
            if seconds_since_last_reset >= 60 {
                stats_guard.messages_per_minute = stats_guard.last_minute_messages;
                stats_guard.last_minute_messages = 0;
                stats_guard.last_minute_start = now;
            }

            // Update memory usage
            let (current_memory, peak_memory) = get_memory_usage(&mut sys, stats_guard.memory_peak_mb);
            stats_guard.memory_usage_mb = current_memory;
            stats_guard.memory_peak_mb = peak_memory;
        }
    });

    info!("Starting dashboard on port {port}");

    // Build routes without CORS first
    let stats_route = warp::path("api")
        .and(warp::path("stats"))
        .and(with_stats())
        .and_then(get_stats);

    let index_route = warp::path("dashboard").and_then(serve_index);

    let routes = stats_route
        .or(index_route)
        .with(warp::cors().allow_any_origin().allow_methods(vec!["GET", "POST"]));

    // Start server in background and return immediately
    let serve_task = warp::serve(routes).run(([0, 0, 0, 0], port));
    tokio::spawn(serve_task);

    Ok(())
}

pub async fn update_stats(message_count: u64) {
    let mut stats = STATS.write().await;
    stats.total_messages += message_count;
    stats.last_minute_messages += message_count;
    stats.last_message_time = Some(Utc::now());
}

fn with_stats() -> impl Filter<Extract = (Arc<RwLock<Stats>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || STATS.clone())
}

async fn get_stats(stats: Arc<RwLock<Stats>>) -> Result<impl Reply, Rejection> {
    let stats_guard = stats.read().await;
    let response = ApiResponse {
        success: true,
        data: Some(stats_guard.clone()),
        message: "Stats retrieved successfully".to_string(),
    };

    Ok(warp::reply::json(&response))
}

async fn serve_index() -> Result<impl Reply, Rejection> {
    let html = include_str!("assets/dashboard.html").to_string();
    Ok(warp::reply::html(html))
}
