use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use log::info;
use service_adsb_rs::cli::Args;
use service_adsb_rs::{dashboard, logger, receiver};

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();
    let config = Args::parse().build_config();
    info!("Please be aware that you are required to comply with local laws and policies.");
    info!("{config:?}");

    let config_arc = Arc::new(config);

    let receiver_config = Arc::clone(&config_arc);
    let dashboard_config = Arc::clone(&config_arc);

    let receiver_handle = tokio::spawn(receiver::read(receiver_config));
    let dashboard_handle = tokio::spawn(dashboard::start(dashboard_config));
    let _ = tokio::try_join!(receiver_handle, dashboard_handle);

    Ok(())
}
