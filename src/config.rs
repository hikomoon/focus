use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub urls: Vec<String>,
    pub repos: Vec<String>,
    pub keywords: Vec<String>,
    pub look_back: u32,
    pub cron: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let cfg = match fs::read_to_string(path) {
            Ok(s) => {
                serde_yaml::from_str(&s)?
            }
            Err(e) => return Err(e).context("reading config file failed").map_err(Into::into),
        };
        Ok(cfg)
    }
}