use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub receiver: Receiver,
    pub service: Service,
}

impl Config {
    pub fn new(receiver: Receiver, service: Service) -> Self {
        Config { receiver, service }
    }
}

#[derive(Debug, Deserialize)]
pub struct Receiver {
    pub ip: String,
    pub port: u16,
}

impl Receiver {
    pub fn new(ip: String, port: u16) -> Self {
        Receiver { ip, port }
    }
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub url: String,
    pub uuid: String,
}

impl Service {
    pub fn new(url: String, uuid: String) -> Self {
        Service { url, uuid }
    }
}
