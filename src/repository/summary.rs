use rocket_dyn_templates::{context, Template};
use crate::config::CONFIG;
use std::fs;
use crate::utils::markdown::md_to_html;
use crate::utils::repo_config::repo_config;
use crate::git::commits::get_commits;
use crate::git::main_branch::main_branch;
use crate::git::tag::get_tag;

#[get("/<repo>")]
pub fn repository(repo: String) -> Option<Template> {
    let mut readme = CONFIG.git_location.clone();
    readme.push(format!("{}.git", repo));
    readme.push("README.md");
    let readme_string = match fs::read(readme) {
        Ok(content) => String::from_utf8_lossy(&content).parse().unwrap_or("".to_string()),
        Err(_) => "".to_string()
    };
    Some(Template::render(
            "repository/summary",
            context! {
                title: repo.clone(),
                repo: repo.clone(),
                config: repo_config(repo.clone()),
                domain: CONFIG.domain.to_string(),
                readme: md_to_html(&readme_string),
                active: "summary",
                preview: get_commits(repo.clone(), 3, None),
                main_branch: main_branch(repo.clone()),
                tag: match get_tag(repo.clone(), 1) {
                    Some(x) => match x.get(0) {
                        Some(x) => Some(x.clone()),
                        None => None
                    },
                    None => None
                },
                payment: CONFIG.payment_link.clone()
            }
        ))
}
