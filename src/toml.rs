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
