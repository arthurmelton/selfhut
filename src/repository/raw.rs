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

#[get("/<repo>/blob/<branch>/<location..>", rank=2)]
pub fn raw(repo: String, branch: String, location: PathBufWithDotfiles) -> Option<Vec<u8>> {
    match file(repo.clone(), branch.clone(), location.get().display().to_string()) {
        Some(file) => {
            Some(file.2) 
        },
        None => { 
            None
        }
    }
}
