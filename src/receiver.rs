use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Result, anyhow};
use base64::Engine;
use base64::engine::general_purpose;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use log::{error, info, warn};
use reqwest::Client;
use tokio::net::TcpStream;
use tokio::time::sleep;

use super::{dashboard, toml::Config};

// Global HTTP client to avoid repeated creation
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: Client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to create HTTP client");
}

pub async fn read(config: Arc<Config>) {
    let mut retry_count = 0;
    const MAX_RETRY_COUNT: u32 = 5;
    const BASE_RETRY_DELAY: u64 = 15;

    loop {
        match TcpStream::connect(format!("{}:{}", config.receiver.ip, config.receiver.port)).await {
            Ok(stream) => {
                info!("Connected to receiver successfully.");
                retry_count = 0; // Reset retry count

                if let Err(e) = handle_connection(stream, &config).await {
                    error!("Connection error: {e}");
                }
            }
            Err(e) => {
                retry_count += 1;
                let delay = if retry_count <= MAX_RETRY_COUNT {
                    BASE_RETRY_DELAY * retry_count as u64
                } else {
                    BASE_RETRY_DELAY * MAX_RETRY_COUNT as u64
                };

                error!("Failed to connect to receiver (attempt {retry_count}/{MAX_RETRY_COUNT}): {e}");
                error!("Retry in {delay} seconds.");
                sleep(Duration::from_secs(delay)).await;
            }
        }
    }
}

async fn handle_connection(stream: TcpStream, config: &Config) -> Result<()> {
    let mut buffer = Vec::with_capacity(8 * 1024); // Initial 8KB buffer

    loop {
        match stream.readable().await {
            Ok(()) => {
                // Dynamically adjust buffer size
                if buffer.capacity() > 64 * 1024 {
                    // Reset to 8KB if over 64KB
                    buffer = Vec::with_capacity(8 * 1024);
                }

                match stream.try_read_buf(&mut buffer) {
                    Ok(0) => {
                        warn!("Stream's read half is closed, attempting to reconnect.");
                        return Ok(()); // Normal close, reconnect
                    }
                    Ok(n) => {
                        if n > 0 && buffer[n - 1] == b'\n' {
                            // Count the number of lines (messages) in this buffer
                            let message_count = buffer[0..n].iter().filter(|&&b| b == b'\n').count() as u64;

                            // Update dashboard stats first
                            dashboard::update_stats(message_count).await;

                            // Send data and handle errors
                            if let Err(e) =
                                send_message((&config.service.url, &config.service.uuid), &buffer[0..n]).await
                            {
                                error!("Failed to send message: {e}");
                                // Continue processing, don't interrupt connection
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(anyhow!("Error reading data: {}", e));
                    }
                }
            }
            Err(e) => {
                warn!("Read error: {e}, retrying...");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        }
    }
}

async fn send_message(service: (&String, &String), src: &[u8]) -> Result<()> {
    let (url, uuid) = service;

    // Compress data with improved error handling
    let compressed_data = {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(src)
            .map_err(|e| anyhow!("Compression write error: {}", e))?;
        encoder
            .finish()
            .map_err(|e| anyhow!("Compression finish error: {}", e))?
    };

    let encoded_data = general_purpose::STANDARD.encode(&compressed_data);
    let post_data = [("from", uuid.as_str()), ("code", encoded_data.as_str())];

    // Use global HTTP client
    let resp = HTTP_CLIENT
        .post(url)
        .form(&post_data)
        .send()
        .await
        .map_err(|e| anyhow!("HTTP request error: {}", e))?;

    match resp.text().await {
        Ok(body) => {
            info!("Upload successful. Response: {body}");
            Ok(())
        }
        Err(err) => Err(anyhow!("Failed to read response: {}", err)),
    }
}
