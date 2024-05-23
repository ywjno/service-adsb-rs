use base64::engine::general_purpose;
use base64::Engine;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::{error, info};
use std::io::{self, Write};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;

use super::toml::Config;

pub async fn read(config: &Config) {
    let stream =
        TcpStream::connect(format!("{}:{}", config.receiver.ip, config.receiver.port)).await;

    loop {
        match stream {
            Ok(ref stream) => {
                info!("Connected to receiver successfully.");
                loop {
                    match stream.readable().await {
                        Ok(()) => {
                            let mut buffer = Vec::with_capacity(1_024 * 8);

                            match stream.try_read_buf(&mut buffer) {
                                Ok(0) => {
                                    error!("Stream's read half is closed, retry.");
                                    continue;
                                }
                                Ok(n) => {
                                    if buffer[n - 1] == b'\n' {
                                        send_message(
                                            (&config.service.url, &config.service.uuid),
                                            &buffer[0..n],
                                        )
                                        .await;
                                    }
                                    continue;
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    continue;
                                }
                                Err(e) => {
                                    error!("Error reading data.\t{}\nAttempting to reconnect.", e);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            error!("Some worry, retry to read.\t{}", e);
                            error!("Retry in 15 seconds.");
                            sleep(Duration::from_secs(15)).await;
                            continue;
                        }
                    }
                }
            }
            Err(ref e) => {
                error!("Failed to connect to receiver.\t{}", e);
                error!("Retry in 15 seconds.");
                sleep(Duration::from_secs(15)).await;
                continue;
            }
        }
    }
}

async fn send_message(service: (&String, &String), src: &[u8]) {
    let (url, uuid) = service;
    let compressed_data = {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(src).unwrap();
        encoder.finish().unwrap()
    };
    let encoded_data = general_purpose::STANDARD.encode(&compressed_data);

    let post_data = [("from", uuid.as_str()), ("code", &encoded_data.as_str())];

    let client = reqwest::Client::new();
    let resp = client.post(url).form(&post_data).send().await;

    match resp {
        Ok(resp) => match resp.text().await {
            Ok(body) => {
                info!("Upload successful.\t{}", body);
            }
            Err(err) => {
                error!("Upload error.\t{}", err);
            }
        },
        Err(err) => {
            error!("Upload error.\t{}", err);
        }
    }
}
