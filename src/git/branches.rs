use crate::config::CONFIG;
use crate::git::commits::{get_commits, Commits};
use serde_derive::Serialize;

pub fn branches(repo_name: String) -> Option<Vec<Branch>> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo_name));
    let repo = git2::Repository::open(repo_path).ok()?;
    let mut branches = Vec::new();
    for i in (repo.branches(Some(git2::BranchType::Local)).ok()?).flatten() {
        if let Ok(Some(name)) = i.0.name() {
            branches.push(Branch {
                branch: name.to_string(),
                commit: match get_commits(repo_name.clone(), 1, Some(name.to_string()), None) {
                    Some(x) => x.first().cloned(),
                    None => None,
                },
            })
        }
    }
    Some(branches)
}

#[derive(Serialize, Clone)]
pub struct Branch {
    branch: String,
    commit: Option<Commits>,
}
