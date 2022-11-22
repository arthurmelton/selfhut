use crate::config::CONFIG;
use rocket_dyn_templates::{context, Template};

use crate::git::commits::get_commits;
use crate::git::file::file;
use crate::git::main_branch::main_branch;
use crate::git::tag::get_tag;
use crate::utils::markdown::md_to_html;
use crate::utils::repo_config::repo_config;

#[get("/<repo>")]
pub fn repository(repo: String) -> Option<Template> {
    let main_branch = main_branch(repo.clone())?;
    let file = file(repo.clone(), main_branch.clone(), "README.md".to_string())?;
    Some(Template::render(
        "repository/summary",
        context! {
            title: repo.clone(),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            readme: md_to_html(&file.1?),
            active: "summary",
            preview: get_commits(repo.clone(), 3, None, None),
            main_branch,
            tag: match get_tag(repo.clone(), 1) {
                Some(x) => match x.get(0) {
                    Some(x) => Some(x.clone()),
                    None => None
                },
                None => None
            },
            payment: CONFIG.payment_link.clone(),
            mailing_list: CONFIG.mailing_list.clone()
        },
    ))
}
