use std::fs;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, crate_description};
use url::Url;

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

    /// Dashboard port
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..), default_value_t=8080)]
    pub dashboard_port: u16,

    /// TOML config file path (look like: ./conf.toml)
    #[arg(long, value_name = "TOML_FILE", value_hint = clap::ValueHint::FilePath)]
    pub config: Option<PathBuf>,
}

impl Args {
    pub fn build_config(&self) -> Config {
        match &self.config {
            Some(config) => self
                .reload_config(config)
                .unwrap_or_else(|e| Args::command().error(ErrorKind::Io, e).exit()),
            None => {
                let receiver_ip = &self.receiver_ip;
                let receiver_port = &self.receiver_port;

                let service_url = self.service_url.as_ref().unwrap_or_else(|| {
                    Args::command()
                        .error(ErrorKind::MissingRequiredArgument, "Server URL is required.")
                        .exit();
                });

                // Validate URL format
                if let Err(e) = Url::parse(service_url) {
                    Args::command()
                        .error(ErrorKind::InvalidValue, format!("Invalid service URL: {e}"))
                        .exit();
                }

                let service_uuid = match &self.service_uuid {
                    Some(service_uuid) => {
                        if !is_valid_uuid(service_uuid) {
                            Args::command()
                                .error(
                                    ErrorKind::InvalidValue,
                                    format!("Invalid UUID format: {service_uuid}. Expected 16 characters."),
                                )
                                .exit();
                        } else {
                            service_uuid
                        }
                    }
                    None => {
                        Args::command()
                            .error(ErrorKind::MissingRequiredArgument, "Server UUID is required.")
                            .exit();
                    }
                };

                Config::new(
                    Receiver::new(receiver_ip.to_string(), *receiver_port),
                    Service::new(service_url.to_string(), service_uuid.to_string()),
                    self.dashboard_port,
                )
            }
        }
    }

    fn reload_config(&self, file_path: &PathBuf) -> Result<Config> {
        let contents = fs::read_to_string(file_path).map_err(|_| {
            anyhow!(
                "The TOML configuration file does not exist. Please check.\n{}",
                file_path.display()
            )
        })?;

        let mut config: Config = toml::from_str(&contents).map_err(|e| {
            anyhow!(
                "An error occurred while parsing the configuration. Please check.\n{}\n{}",
                file_path.display(),
                e
            )
        })?;

        // Override dashboard port from command line if specified
        if self.dashboard_port != 8080 {
            config.dashboard_port = self.dashboard_port;
        }

        // Validate loaded configuration
        validate_config(&config)?;

        Ok(config)
    }
}

/// Validate UUID format
fn is_valid_uuid(uuid: &str) -> bool {
    // Check if length is 16 characters
    if uuid.len() != 16 {
        return false;
    }

    // Check if only contains valid characters (letters and numbers)
    uuid.chars().all(|c| c.is_alphanumeric())
}

/// Validate configuration
fn validate_config(config: &Config) -> Result<()> {
    // Validate port range
    if config.receiver.port == 0 {
        return Err(anyhow!("Receiver port cannot be 0"));
    }

    // Validate service URL
    if let Err(e) = Url::parse(&config.service.url) {
        return Err(anyhow!("Invalid service URL: {}", e));
    }

    // Validate UUID
    if !is_valid_uuid(&config.service.uuid) {
        return Err(anyhow!("Invalid UUID format: {}", config.service.uuid));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_config() -> Config {
        Config::new(
            Receiver::new("127.0.0.1".to_string(), 30_003),
            Service::new("https://example.com/upload".to_string(), "ABCDEF1234567890".to_string()),
            8080,
        )
    }

    #[test]
    fn uuid_must_be_16_alphanumeric_characters() {
        assert!(is_valid_uuid("ABCDEF1234567890"));
        assert!(!is_valid_uuid("short"));
        assert!(!is_valid_uuid("ABCDEF12345678901"));
        assert!(!is_valid_uuid("ABCDEF12345678!!"));
    }

    #[test]
    fn valid_config_passes_validation() {
        assert!(validate_config(&valid_config()).is_ok());
    }

    #[test]
    fn config_validation_rejects_invalid_service_url() {
        let mut config = valid_config();
        config.service.url = "not-a-url".to_string();

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn config_validation_rejects_invalid_uuid() {
        let mut config = valid_config();
        config.service.uuid = "bad".to_string();

        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn config_validation_rejects_zero_receiver_port() {
        let mut config = valid_config();
        config.receiver.port = 0;

        assert!(validate_config(&config).is_err());
    }
}
