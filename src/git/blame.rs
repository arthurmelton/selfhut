use crate::config::CONFIG;
use crate::git::commits::{get_commits, Commits};
use serde_derive::Serialize;
use std::ops::Range;
use std::path::Path;

pub fn blame<'a>(repo: String, branch: String, path: String) -> Option<Vec<Blame>> {
    if branch.contains(":") {
        return None;
    }
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo_clone = repo.clone();
    let repo = git2::Repository::open(repo_path).ok()?;
    let blame = repo.blame_file(Path::new(&path), None).ok()?;
    let mut blames = Vec::new();
    for i in blame.iter() {
        blames.push(Blame {
            commit: (*get_commits(
                repo_clone.clone(),
                1,
                Some(i.final_commit_id().to_string()),
                None,
            )?
            .first()?)
            .clone(),
            lines: (0..i.lines_in_hunk() - 1).collect(),
        });
    }
    Some(blames)
}

#[derive(Serialize, Clone)]
pub struct Blame {
    commit: Commits,
    lines: Vec<usize>,
}
