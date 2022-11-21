use crate::config::CONFIG;
use crate::git::commits::get_commits;
use crate::git::file::{file, files};
use crate::git::main_branch::main_branch;
use crate::utils::repo_config::repo_config;
use crate::PathBufWithDotfiles;
use rocket_dyn_templates::{context, Template};
use std::ffi::OsStr;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[get("/<repo>/log?<from>", rank = 2)]
pub fn log_main(repo: String, from: Option<String>) -> Option<Template> {
    let main_branch = main_branch(repo.clone())?;
    let commits = get_commits(repo.clone(), 21, from, None);
    let last_commit = match commits {
        Some(ref commits) => {
            if commits.len() == 21 {
                Some(commits[19].commit_hash.clone())
            } else {
                None
            }
        }
        None => None,
    };
    let commits = match commits {
        Some(mut commits) => {
            if commits.len() == 21 {
                commits.pop();
            }
            Some(commits)
        }
        None => None,
    };
    Some(Template::render(
        "repository/log",
        context! {
            title: format!("/ :: {}", repo.clone()),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            active: "log",
            commits,
            branch: main_branch.clone(),
            current_dir_file: "/",
            current_dir: "/",
            payment: CONFIG.payment_link.clone(),
            last_commit,
        },
    ))
}

#[get("/<repo>/log/<branch>/<location..>?<from>", rank = 2)]
pub fn log(
    repo: String,
    branch: String,
    location: PathBufWithDotfiles,
    from: Option<String>,
) -> Option<Template> {
    let commits = get_commits(
        repo.clone(),
        21,
        from,
        Some(format!("{}", location.get().display()).replace("//", "/")),
    );
    let last_commit = match commits {
        Some(ref commits) => {
            if commits.len() == 21 {
                Some(commits[19].commit_hash.clone())
            } else {
                None
            }
        }
        None => None,
    };
    let commits = match commits {
        Some(mut commits) => {
            if commits.len() == 21 {
                commits.pop();
            }
            Some(commits)
        }
        None => None,
    };
    Some(Template::render(
        "repository/log",
        context! {
            title: format!("/{} :: {}", location.get().display(), repo.clone()),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            active: "log",
            commits,
            branch: branch.clone(),
            current_dir_file: format!("/{}/", location.get().display()).replace("//", "/"),
            current_dir: format!("/{}", location.get().display()),
            payment: CONFIG.payment_link.clone(),
            last_commit,
        },
    ))
}
