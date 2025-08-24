use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db: String,
    pub interval: u64,
    pub mc: ServerConfig,
    pub web: WebConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub java: bool,
    pub addr: String,
}

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
    pub port: u16,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let s = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&s)?;
        Ok(config)
    }
}
