use clap::Parser;
use log::info;

use service_adsb_rs::{cli::Args, logger, receiver};

#[tokio::main]
async fn main() {
    logger::init();
    let config = Args::parse().build_config();
    info!("Please be aware that you are required to comply with local laws and policies.");
    info!("{:?}", config);
    receiver::read(&config).await;
}
