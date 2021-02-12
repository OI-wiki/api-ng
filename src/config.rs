use std::{
    net::{IpAddr, Ipv6Addr},
    usize,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub repo_path: String,
    pub preview_lines: i32,
    pub lru_cap: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            port: 2783,
            repo_path: "./OI-Wiki".to_string(),
            preview_lines: 5,
            lru_cap: 50,
        }
    }
}

impl Config {
    pub async fn load(path: Option<&str>) -> Option<Self> {
        let file = tokio::fs::read_to_string(path.unwrap_or("config.yaml")).await;
        if let Ok(str) = file {
            let mut res: Self = serde_yaml::from_str(&str).unwrap();
            res.repo_path = res.repo_path.trim_end_matches('/').to_string();
            return Some(res);
        }
        None
    }
}
