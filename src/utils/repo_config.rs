use crate::git::file::file;
use crate::git::main_branch::main_branch;
use serde_derive::{Deserialize, Serialize};

pub fn repo_config(repo: String) -> RepoConfig {
    let main_branch = main_branch(repo.clone()).unwrap_or_default();
    let content = match file(repo, main_branch, "repo.toml".to_string()) {
        Some(file) => file.1.unwrap_or_default(),
        None => "".to_string(),
    };
    match toml::from_str(&content) {
        Ok(x) => x,
        Err(_) => RepoConfig {
            description: None,
            website: None,
        },
    }
}

#[derive(Deserialize, Serialize)]
pub struct RepoConfig {
    pub description: Option<String>,
    pub website: Option<String>,
}
