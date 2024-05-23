use clap::error::ErrorKind;
use clap::{arg, crate_description, CommandFactory, Parser};
use std::fs;
use std::path::PathBuf;

use super::toml::{Config, Receiver, Service};

#[derive(Parser, Debug)]
#[command(version, about = crate_description!(), long_about = None)]
pub struct Args {
    /// Receiver ip
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub receiver_ip: String,

    /// Receiver port
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t=30_003)]
    pub receiver_port: u16,

    /// Service url
    #[arg(long)]
    pub service_url: Option<String>,

    /// Service uuid
    #[arg(long)]
    pub service_uuid: Option<String>,

    /// TOML config file path (look like: ./conf.toml)
    #[arg(long, value_name = "TOML_FILE", value_hint = clap::ValueHint::FilePath)]
    pub config: Option<PathBuf>,
}

impl Args {
    pub fn build_config(&self) -> Config {
        match &self.config {
            Some(config) => self
                .reload_config(&config)
                .unwrap_or_else(|e| Args::command().error(ErrorKind::Io, e).exit()),
            None => {
                let receiver_ip = &self.receiver_ip;
                let receiver_port = &self.receiver_port;
                let service_url = self.service_url.as_ref().unwrap_or_else(|| {
                    Args::command()
                        .error(
                            ErrorKind::MissingRequiredArgument,
                            "Server URL is required.",
                        )
                        .exit();
                });
                let service_uuid = match &self.service_uuid {
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

                Config::new(
                    Receiver::new(receiver_ip.to_string(), *receiver_port),
                    Service::new(service_url.to_string(), service_uuid.to_string()),
                )
            }
        }
    }

    fn reload_config(&self, file_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
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
}
