use crate::git::repos::get_repos;
use crate::config::CONFIG;

use rocket_dyn_templates::{Template, context};

#[get("/?<page>")]
pub fn index(page: Option<usize>) -> Option<Template> {
    let repos = get_repos()?;
    let page = page.unwrap_or(0);
    let last_item = if repos.len() > (page+1)*25 {
        (page+1)*25
    }
    else {
        repos.len()
    };
    Some(Template::render("index", context! {
        title: "Me",
        user: CONFIG.name.clone(),
        description: markdown::to_html(&CONFIG.description.clone()),
        repos: repos[page*25..last_item].to_vec(),
    }))
}
