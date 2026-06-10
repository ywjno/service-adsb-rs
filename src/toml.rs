use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub receiver: Receiver,
    pub service: Service,
    #[serde(default = "default_dashboard_port")]
    pub dashboard_port: u16,
}

fn default_dashboard_port() -> u16 {
    8080
}

impl Config {
    pub fn new(receiver: Receiver, service: Service, dashboard_port: u16) -> Self {
        Config {
            receiver,
            service,
            dashboard_port,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Receiver {
    pub ip: String,
    pub port: u16,
}

impl Receiver {
    pub fn new(ip: String, port: u16) -> Self {
        Receiver { ip, port }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Service {
    pub url: String,
    pub uuid: String,
}

impl Service {
    pub fn new(url: String, uuid: String) -> Self {
        Service { url, uuid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_uses_default_dashboard_port_when_missing() {
        let config: Config = ::toml::from_str(
            r#"
            [receiver]
            ip = "127.0.0.1"
            port = 30003

            [service]
            url = "https://example.com/upload"
            uuid = "ABCDEF1234567890"
            "#,
        )
        .unwrap();

        assert_eq!(config.dashboard_port, 8080);
    }
}
