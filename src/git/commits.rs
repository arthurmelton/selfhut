use crate::config::CONFIG;
use chrono::{Duration, Utc};
use chrono_humanize::HumanTime;
use serde_derive::Serialize;
use std::borrow::Cow;

pub fn get_commits(
    repo_name: String,
    ammount: usize,
    after: Option<String>,
    path: Option<String>,
) -> Option<Vec<Commits>> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo_name));
    let repo = git2::Repository::open(repo_path).ok()?;
    let mut revwalk = repo.revwalk().ok()?;

    revwalk.set_sorting(git2::Sort::TIME).ok()?;
    match after {
        Some(id) => revwalk.push(git2::Oid::from_str(id.as_str()).ok()?).ok()?,
        None => revwalk.push_head().ok()?,
    }
    let mut commits = Vec::new();
    let mut i = 0;
    let mut commit = revwalk.next();
    while i < ammount && commit.is_some() && commit.as_ref().unwrap().is_ok() {
        let commit_rev = commit.unwrap().unwrap();
        let repo_commit = repo.find_commit(commit_rev);
        match repo_commit {
            Ok(repo_commit) => {
                let mut allowed = true;
                if path.is_some() {
                    let path_value = path.as_ref().unwrap();
                    match repo_commit.tree() {
                        Ok(tree) => {
                                match repo_commit.parent(0) {
                                    Ok(parent) => {
                                        match parent.tree() {
                                            Ok(parent_tree) => {
                                                match repo.diff_tree_to_tree(Some(&tree), Some(&parent_tree), None) {
                                                    Ok(diff) => {
                                                        let files = diff.deltas()
                                                            .map(|i| i.new_file().path())
                                                            .filter(|i| i.is_some())
                                                            .map(|i| i.unwrap().to_string_lossy())
                                                            .filter(|i| i.starts_with(path_value))
                                                            .count();
                                                        allowed = files > 0;
                                                    },
                                                    Err(_) => {}
                                                }

                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    Err(_) => {}
                                }
                        },
                        Err(_) => {}
                    }
                }
                if allowed {
                    let time = Duration::seconds(repo_commit.time().seconds() - Utc::now().timestamp());
                    commits.push(Commits {
                        commit_hash: commit_rev.to_string(),
                        commit: repo_commit.message().unwrap_or("").to_string(),
                        commitie: repo_commit
                            .committer()
                            .name()
                            .unwrap_or("Unknown")
                            .to_string(),
                        commit_hash_short: commit_rev.to_string()[..8].to_string(),
                        time_utc: {
                            let date = chrono::naive::NaiveDateTime::from_timestamp(
                                Utc::now().timestamp() + time.num_seconds(),
                                0,
                            );
                            format!(
                                "{} {} UTC",
                                date.date().format("%Y-%m-%d"),
                                date.time().format("%H:%M:%S")
                            )
                        },
                        time_relitive: HumanTime::from(time).to_string(),
                    });
                    i += 1;
                }
            }
            Err(_) => {}
        }
        commit = revwalk.next();
    }
    Some(commits)
}

#[derive(Serialize, Clone)]
pub struct Commits {
    commit: String,
    commit_hash: String,
    commit_hash_short: String,
    commitie: String,
    time_utc: String,
    time_relitive: String,
}
