use crate::git::file::file;
use crate::PathBufWithDotfiles;

#[get("/<repo>/blob/<branch>/<location..>", rank = 2)]
pub fn raw(repo: String, branch: String, location: PathBufWithDotfiles) -> Option<Vec<u8>> {
    match file(repo, branch, location.get().display().to_string()) {
        Some(file) => Some(file.2),
        None => None,
    }
}
