use std::sync::{Arc, Once};

use anyhow::Result;
use clap::Parser;
use log::info;
use service_adsb_rs::cli::Args;
use service_adsb_rs::{dashboard, logger, receiver};

#[tokio::main]
async fn main() -> Result<()> {
    static INIT_RUSTLS_PROVIDER: Once = Once::new();
    INIT_RUSTLS_PROVIDER.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");
    });

    logger::init();
    let config = Args::parse().build_config();
    info!("Please be aware that you are required to comply with local laws and policies.");
    info!("{config:?}");

    let config_arc = Arc::new(config);

    let receiver_config = Arc::clone(&config_arc);
    let dashboard_config = Arc::clone(&config_arc);

    let receiver_handle = tokio::spawn(receiver::read(receiver_config));
    let dashboard_handle = tokio::spawn(dashboard::start(dashboard_config));
    let (_, dashboard_result) = tokio::try_join!(receiver_handle, dashboard_handle)?;
    dashboard_result?;

    Ok(())
}
