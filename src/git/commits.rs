use crate::config::CONFIG;
use crate::git::diffs::diffs;
use chrono::{Duration, Utc};
use chrono_humanize::HumanTime;
use serde_derive::Serialize;

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
        Some(id) => match git2::Oid::from_str(id.as_str()) {
            Ok(x) => revwalk.push(x).ok()?,
            Err(_) => {
                let oid = match repo.find_branch(&id, git2::BranchType::Local) {
                    Ok(x) => x.get().target(),
                    Err(_) => {
                        let mut tag = None;
                        let tag_name = id.as_bytes();
                        let _ = repo.tag_foreach(|x,y| {
                            if &y[10..] == tag_name {
                                tag = Some(x);
                                return false;
                            }
                            true
                        });
                        tag
                    }
                };
                match oid {
                    Some(x) => revwalk.push(x).ok()?,
                    None => revwalk.push_head().ok()?
                }
            }
        }
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
                    match diffs(repo_commit.clone(), &repo) {
                        Some(diff) => {
                            let files = diff
                                .deltas()
                                .map(|i| i.new_file().path())
                                .filter(|i| i.is_some())
                                .map(|i| i.unwrap().to_string_lossy())
                                .filter(|i| i.starts_with(path_value))
                                .count();
                            allowed = files > 0;
                        }
                        None => {}
                    }
                }
                if allowed {
                    let time =
                        Duration::seconds(repo_commit.time().seconds() - Utc::now().timestamp());
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
                        commit_body: repo_commit.body().unwrap_or("").to_string(),
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
    pub commit: String,
    pub commit_hash: String,
    pub commit_hash_short: String,
    pub commitie: String,
    pub time_utc: String,
    pub time_relitive: String,
    pub commit_body: String,
}
