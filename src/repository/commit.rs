use crate::config::CONFIG;

use crate::git::commits::get_commits;
use crate::git::diffs::diffs;

use git2::DiffLineType::*;
use serde_derive::Serialize;

use crate::utils::repo_config::repo_config;

use git2::Delta::*;
use rocket_dyn_templates::{context, Template};

#[get("/<repo>/commit/<oid>", rank = 2)]
pub fn commit(repo: String, oid: String) -> Option<Template> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let repo_clone = repo.clone();
    let repo = git2::Repository::open(repo_path).ok()?;
    let commit = repo.find_commit(git2::Oid::from_str(&oid).ok()?).ok()?;
    let diff = diffs(commit.clone(), &repo)?;
    let stats = diff.stats().ok()?;
    Some(Template::render(
        "repository/commit",
        context! {
            title: format!("{} :: {}", oid, repo_clone.clone()),
            repo: repo_clone.clone(),
            config: repo_config(repo_clone.clone()),
            domain: CONFIG.domain.to_string(),
            active: "log",
            payment: CONFIG.payment_link.clone(),
            mailing_list: CONFIG.mailing_list.clone(),
            commit: match get_commits(repo_clone.clone(), 1, Some(oid.clone()), None) {
                Some(x) => match x.clone().get(0) {
                    Some(x) => Some(x.clone()),
                    None => None
                }
                None => None
            },
            parent: match commit.parent_id(0) {
                Ok(parent) => {
                    match get_commits(repo_clone.clone(), 1, Some(parent.to_string()), None) {
                        Some(x) => match x.first() {
                            Some(x) => Some(x.clone()),
                            None => None
                        },
                        None => None
                    }
                },
                Err(_) => None
            },
            files_changed: stats.files_changed(),
            insertions: stats.insertions(),
            deletions: stats.deletions(),
            files: {
                let mut items = Vec::new();
                let mut x = 0;
                for i in diff.deltas() {
                    let patch = git2::Patch::from_diff(&diff, x).ok()??;
                    let hunk_n = patch.num_hunks();
                    let mut hunks = Vec::new();
                    for y in 0..hunk_n {
                        let hunk = patch.hunk(y).ok()?;
                        let line_n = patch.num_lines_in_hunk(y).ok()?;
                        let mut lines = Vec::new();
                        for z in 0..line_n {
                            let line = patch.line_in_hunk(y, z).ok()?;
                            lines.push(Lines {
                                line_n: z,
                                class: match line.origin_value() {
                                        Addition => "text-success",
                                        Deletion => "text-danger",
                                        AddEOFNL => "text-success",
                                        DeleteEOFNL => "text-danger",
                                        _ => ""
                                }.to_string(),
                                types: line.origin(),
                                line: std::str::from_utf8(line.content()).ok()?.to_string()
                            });
                        }
                        let header = std::str::from_utf8(hunk.0.header()).ok()?.to_string();
                        let mut first_line = header.split("@@").collect::<Vec<&str>>();
                        for _ in 0..2 {
                            first_line.remove(0);
                        }
                        hunks.push(Hunk {
                            hunk_n: y,
                            first: hunk.0.old_start(),
                            second: hunk.0.old_lines(),
                            third: hunk.0.new_start(),
                            fourth: hunk.0.new_lines(),
                            first_line: first_line.join("@@"),
                            lines
                        });
                    }
                    items.push(Files {
                        beging_path: (*i.old_file().path()?).display().to_string(),
                        end_path: (*i.new_file().path()?).display().to_string(),
                        status: match i.status() {
                            Added => 'A',
                            Deleted => 'D',
                            Modified => 'M',
                            Renamed => 'R',
                            Copied => 'C',
                            Ignored => 'I',
                            Typechange => 'T',
                            Conflicted => 'C',
                            _ => ' ',
                        },
                        insertions: patch.line_stats().ok()?.1,
                        deletions: patch.line_stats().ok()?.2,
                        hunks
                    });
                    x+=1;
                }
                items
            },
        },
    ))
}

#[get("/<repo>/patch/<oid>", rank = 2)]
pub fn patch(repo: String, oid: String) -> Option<String> {
    let mut repo_path = CONFIG.git_location.clone();
    repo_path.push(format!("{}.git", repo));
    let _repo_clone = repo.clone();
    let repo = git2::Repository::open(repo_path).ok()?;
    let commit = repo.find_commit(git2::Oid::from_str(&oid).ok()?).ok()?;
    let _diff = diffs(commit, &repo)?;
    None
}

#[derive(Serialize, Clone)]
pub struct Files {
    beging_path: String,
    end_path: String,
    status: char,
    insertions: usize,
    deletions: usize,
    hunks: Vec<Hunk>,
}

#[derive(Serialize, Clone)]
pub struct Hunk {
    hunk_n: usize,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    first_line: String,
    lines: Vec<Lines>,
}

#[derive(Serialize, Clone)]
pub struct Lines {
    line_n: usize,
    class: String,
    types: char,
    line: String,
}
