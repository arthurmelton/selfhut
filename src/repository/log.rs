use rocket_dyn_templates::{context, Template};
use crate::config::CONFIG;
use crate::utils::repo_config::repo_config;
use crate::git::commits::get_commits;
use crate::git::main_branch::main_branch;
use crate::git::file::{files, file};
use crate::PathBufWithDotfiles;
use std::ffi::OsStr;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::LinesWithEndings;
use syntect::html::highlighted_html_for_string;
use std::path::Path;

#[get("/<repo>/log", rank=2)]
pub fn tree_main(repo: String) -> Option<Template> {
    let main_branch = main_branch(repo.clone())?;
    Some(Template::render(
            "repository/log",
            context! {
                title: format!("/ :: {}", repo.clone()),
                repo: repo.clone(),
                config: repo_config(repo.clone()),
                domain: CONFIG.domain.to_string(),
                active: "tree",
                commits: get_commits(repo.clone(), 1, None),
                branch: main_branch.clone(),
                current_dir_file: "/",
                current_dir: "/"
            }
        ))
}

#[get("/<repo>/log/<branch>/<location..>", rank=2)]
pub fn tree(repo: String, branch: String, location: PathBufWithDotfiles) -> Option<Template> {
        Some(Template::render(
            "repository/log",
            context! {
                title: format!("/{} :: {}", location.get().display(), repo.clone()),
                repo: repo.clone(),
                config: repo_config(repo.clone()),
                domain: CONFIG.domain.to_string(),
                active: "tree",
                commits: get_commits(repo.clone(), 1, None),
                branch: branch.clone(),
                current_dir_file: format!("/{}/", location.get().display()).replace("//", "/"),
                current_dir: format!("/{}", location.get().display()),
            }
        ))
    }
}
