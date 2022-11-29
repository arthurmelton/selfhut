use crate::config::CONFIG;
use git2::ObjectType;
use humansize::{format_size, DECIMAL};
use serde_derive::Serialize;

use std::path::PathBuf;

pub fn files(repo: String, branch: String, path: String) -> Option<Vec<File>> {
    if branch.contains(':') {
        return None;
    }
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo = git2::Repository::open(repo_path).ok()?;
    let obj = repo.revparse_single(&format!("{}:{}", branch, path)).ok()?;
    let mut returns = Vec::new();
    match obj.kind() {
        Some(ObjectType::Tree) => {
            let tree = obj.as_tree()?;
            for entry in tree.iter() {
                returns.push(File {
                    name: entry.name()?.to_string(),
                    size: match entry.kind() {
                        Some(ObjectType::Blob) => Some(format_size(
                            entry.to_object(&repo).ok()?.into_blob().ok()?.size(),
                            DECIMAL,
                        )),
                        _ => None,
                    },
                    file_type: match entry.kind() {
                        Some(ObjectType::Blob) => FileType::blob,
                        _ => FileType::tree,
                    },
                    properties: unix_mode::to_string(entry.filemode() as u32),
                    properties_int: entry.filemode(),
                })
            }
        }
        _ => {
            return None;
        }
    }
    Some(returns)
}

pub fn file(repo: String, branch: String, path: String) -> Option<(File, Option<String>, Vec<u8>)> {
    if branch.contains(':') {
        return None;
    }
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo = git2::Repository::open(repo_path).ok()?;
    let obj = repo.revparse_single(&format!("{}:{}", branch, path)).ok()?;
    match obj.kind() {
        Some(ObjectType::Blob) => {
            let blob = obj.as_blob()?;
            let mut path_to_folder = PathBuf::from(path);
            path_to_folder.pop();
            let path_to_folder = path_to_folder.into_os_string().to_str()?.to_string();
            let tree = repo
                .revparse_single(&format!("{}:{}", branch, path_to_folder))
                .ok()?;
            let tree = tree.as_tree()?;
            let tree_item = tree.get_id(blob.id())?;
            let file = File {
                name: tree_item.name()?.to_string(),
                size: Some(format_size(blob.size(), DECIMAL)),
                file_type: FileType::blob,
                properties: unix_mode::to_string(tree_item.filemode() as u32),
                properties_int: tree_item.filemode(),
            };
            let content = match blob.is_binary() {
                false => Some(String::from_utf8_lossy(blob.content()).into_owned()),
                true => None,
            };
            return Some((file, content, blob.content().iter().map(|x| *x).collect()));
        }
        _ => None,
    }
}

#[derive(Serialize)]
pub struct File {
    pub name: String,
    pub size: Option<String>,
    pub file_type: FileType,
    pub properties: String,
    pub properties_int: i32,
}

#[derive(Serialize)]
#[allow(non_camel_case_types)]
pub enum FileType {
    blob,
    tree,
}
