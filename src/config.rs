use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    thread_count: usize,
    ip: String,
}

impl Config {
    /// Parse the config at `path`
    pub fn new(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;

        let config: Config = toml::from_str(&contents.clone())?;

        Ok(config)
    }

    pub fn thread_count(&self) -> usize {
        self.thread_count
    }

    pub fn ip(&self) -> &String {
        &self.ip
    }
}