use rocket::fs::NamedFile;
use crate::config::CONFIG;
use crate::PathBufWithDotfiles;
use std::path::Path;

#[get("/<repo>/<path..>", rank=3)]
pub async fn clone(repo: String, path: PathBufWithDotfiles) -> Option<NamedFile> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    repo_path.push(path.get());
    NamedFile::open(repo_path).await.ok()
}
