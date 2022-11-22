use crate::config::CONFIG;
use crate::git::blame::blame;
use crate::git::commits::get_commits;
use crate::git::file::file;

use crate::utils::repo_config::repo_config;
use crate::PathBufWithDotfiles;
use rocket_dyn_templates::{context, Template};
use std::ffi::OsStr;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[get("/<repo>/blame/<branch>/<location..>", rank = 2)]
pub fn blames(repo: String, branch: String, location: PathBufWithDotfiles) -> Option<Template> {
    let blames = blame(
        repo.clone(),
        branch.clone(),
        location.get().display().to_string(),
    );
    let file = file(
        repo.clone(),
        branch.clone(),
        location.get().display().to_string(),
    )?;
    let mut content = "".to_string();
    let mut lines: Vec<usize> = (1..1).collect();
    if file.1.is_some() {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = match ps.find_syntax_by_extension(
            Path::new(&file.0.name)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap_or("txt"),
        ) {
            Some(x) => x,
            None => ps.find_syntax_by_extension("txt").unwrap(),
        };
        let s = file.1.as_ref().unwrap();
        lines = (1..s.lines().count() + 1).collect();
        content =
            highlighted_html_for_string(s, &ps, syntax, &ts.themes["Solarized (light)"]).ok()?
    }
    Some(Template::render(
        "repository/blame",
        context! {
            title: format!("/{} :: {}", location.get().display(), repo.clone()),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            active: "tree",
            commit: match get_commits(repo.clone(), 1, None, Some(format!("{}", location.get().display()).replace("//", "/"))) {
                Some(x) => match x.clone().get(0) {
                    Some(x) => Some(x.clone()),
                    None => None
                }
                None => None
            },
            files: file,
            content,
            lines,
            fluid: "true",
            blame: blames,
            branch: branch.clone(),
            current_dir_file: format!("/{}/", location.get().display()).replace("//", "/"),
            current_dir: format!("/{}", location.get().display()),
            payment: CONFIG.payment_link.clone()
        },
    ))
}
