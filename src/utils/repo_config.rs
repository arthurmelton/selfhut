use serde_derive::{Deserialize, Serialize};
use crate::config::CONFIG;
use std::fs;

pub fn repo_config(repo: String) -> RepoConfig {
    let mut config = CONFIG.git_location.clone();
    config.push(format!("{}.git", repo));
    config.push("repo.toml");
    let config_string = match fs::read(config) {
        Ok(content) => String::from_utf8_lossy(&content).parse().unwrap_or("".to_string()),
        Err(_) => "".to_string()
    };
    match toml::from_str(&config_string) {
        Ok(x) => x,
        Err(_) => RepoConfig {
            description: None,
            website: None
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RepoConfig {
    pub description: Option<String>,
    pub website: Option<String>,
}
