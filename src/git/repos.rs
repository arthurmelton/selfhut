use std::fs;
use serde_derive::Serialize;

pub fn get_repos() -> Option<Vec<Repo>> {
    let home = dirs::home_dir()?;
    Some(fs::read_dir(home.clone()).ok()?
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap())
        .filter(|path| path.file_type().is_ok())
        .filter(|path| path.file_type().unwrap().is_dir())
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
        .collect::<Vec<Repo>>())
}

#[derive(Serialize, Clone)]
pub struct Repo {
    name: String,
    description: String,
}
