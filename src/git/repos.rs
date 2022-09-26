use std::fs;
use serde_derive::Serialize;
use crate::config::CONFIG;

pub fn get_repos() -> Option<Vec<Repo>> {
    let home = CONFIG.git_location.clone();
    Some(sort_modified(fs::read_dir(home.clone()).ok()?
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .filter(|path| path.file_type().is_ok())
        .filter(|path| path.file_type().unwrap().is_dir())
        .filter(|path| path.metadata().is_ok())
        .filter(|path| path.metadata().unwrap().modified().is_ok())
        .map(|path| path.file_name().into_string())
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .filter(|path| !path.starts_with("."))
        .filter(|path| path.ends_with(".git"))
        .map(|path| {
            let mut description_path = home.clone();
            description_path.push(path.clone());
            description_path.push("DESCRIPTION");
            let description = match fs::read(description_path.clone()) {
                Ok(content) => {
                    String::from_utf8_lossy(&content).parse().unwrap_or("".to_string())
                },
                Err(_) => "".to_string()
            };
            Repo {
                name: path[0..path.len() - 4].to_string(),
                description
            }
        })
        .collect::<Vec<Repo>>()))
}

#[derive(Serialize, Clone)]
pub struct Repo {
    pub name: String,
    pub description: String,
}

fn sort_modified(repos: Vec<Repo>) -> Vec<Repo> {
    let mut repos = repos.clone();
    let home = CONFIG.git_location.clone();
    repos.sort_by(|a,b| {
        let mut a_loc = home.clone();
        a_loc.push(format!("{}.git", a.name));
        let mut b_loc = home.clone();
        b_loc.push(format!("{}.git", b.name));
        b_loc.metadata().unwrap().modified().unwrap().partial_cmp(&a_loc.metadata().unwrap().modified().unwrap()).unwrap()
    });
    repos
}
