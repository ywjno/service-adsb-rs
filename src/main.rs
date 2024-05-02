use base64::engine::general_purpose;
use base64::Engine;
use clap::error::ErrorKind;
use clap::{arg, crate_description, CommandFactory, Parser};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use log::{error, info};
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct Config {
    receiver: Receiver,
    service: Service,
}

#[derive(Debug, Deserialize)]
struct Receiver {
    ip: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Service {
    url: String,
    uuid: String,
}

#[derive(Parser, Debug)]
#[command(version, about = crate_description!(), long_about = None)]
struct Args {
    /// Receiver ip
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    receiver_ip: String,

    /// Receiver port
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t=30_003)]
    receiver_port: u16,

    /// Service url
    #[arg(long)]
    service_url: Option<String>,

    /// Service uuid
    #[arg(long)]
    service_uuid: Option<String>,

    /// TOML config file path (look like: ./conf.toml)
    #[arg(long, value_name = "TOML_FILE", value_hint = clap::ValueHint::FilePath)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    init_logger();
    let args = Args::parse();
    let conf = parse(&args);
    info!("Please be aware that you are required to comply with local laws and policies.");
    info!("{:?}", conf);
    read_receiver(&conf).await;
}

fn init_logger() {
    env_logger::builder()
        .parse_env(env_logger::Env::new().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Stdout)
        .init();
}

fn parse(args: &Args) -> Config {
    match &args.config {
        Some(config) => reload_config(&config)
            .unwrap_or_else(|e| Args::command().error(ErrorKind::Io, e).exit()),
        None => {
            let receiver_ip = &args.receiver_ip;
            let receiver_port = args.receiver_port;
            let service_url = args.service_url.as_ref().unwrap_or_else(|| {
                Args::command()
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "Server URL is required.",
                    )
                    .exit();
            });
            let service_uuid = match &args.service_uuid {
                Some(service_uuid) => {
                    if service_uuid.len() != 16 {
                        Args::command()
                            .error(ErrorKind::NoEquals, "Did not meet the standard length of 16 characters for the Server UUID.")
                            .exit();
                    } else {
                        service_uuid
                    }
                }
                None => {
                    Args::command()
                        .error(
                            ErrorKind::MissingRequiredArgument,
                            "Server UUID is required.",
                        )
                        .exit();
                }
            };

            Config {
                receiver: Receiver {
                    ip: receiver_ip.to_string(),
                    port: receiver_port,
                },
                service: Service {
                    url: service_url.to_string(),
                    uuid: service_uuid.to_string(),
                },
            }
        }
    }
}

fn reload_config(file_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    match fs::read_to_string(file_path) {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!(
                "An error occurred while parsing the configuration. Please check.\n{}\n{}",
                &file_path.display(),
                e
            )
            .into()),
        },
        Err(_) => Err(format!(
            "The TOML configuration file does not exist. Please check.\n{}",
            &file_path.display()
        )
        .into()),
    }
}

async fn read_receiver(conf: &Config) {
    let stream = TcpStream::connect(format!("{}:{}", conf.receiver.ip, conf.receiver.port)).await;

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
                                        send_message(&conf.service, &buffer[0..n]).await;
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

async fn send_message(service: &Service, src: &[u8]) {
    let compressed_data = {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(src).unwrap();
        encoder.finish().unwrap()
    };
    let encoded_data = general_purpose::STANDARD.encode(&compressed_data);

    let post_data = [
        ("from", &service.uuid.as_str()),
        ("code", &encoded_data.as_str()),
    ];

    let client = reqwest::Client::new();
    let resp = client.post(&service.url).form(&post_data).send().await;

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
