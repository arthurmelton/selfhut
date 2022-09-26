use crate::config::CONFIG;
use crate::git::repos::get_repos;
use crate::git::repos::Repo;

use rocket_dyn_templates::{context, Template};

const PAGE_REPO_COUNT: usize = 10;

#[get("/?<page>&<search>")]
pub fn index(page: Option<usize>, search: Option<String>) -> Option<Template> {
    let mut repos = get_repos()?;
    let page = page.unwrap_or(1);
    if search.is_some() {
        repos = repos
            .iter()
            .filter(|repo| {
                repo.name.contains(&*search.as_ref().unwrap())
                    || repo.description.contains(&*search.as_ref().unwrap())
            })
            .map(|repo| repo.clone())
            .collect::<Vec<Repo>>();
    }
    let total_page = if repos.len() > 0 {
        (repos.len() - 1) / PAGE_REPO_COUNT + 1
    } else {
        1
    };
    let last_item = if repos.len() > page * PAGE_REPO_COUNT {
        page * PAGE_REPO_COUNT
    } else {
        repos.len()
    };
    Some(Template::render(
        "index",
        context! {
            title: "Me",
            user: CONFIG.name.clone(),
            description: markdown::to_html(&CONFIG.description.clone()),
            repos: repos[(page-1)*PAGE_REPO_COUNT..last_item].to_vec(),
            search: search.unwrap_or("".to_string()),
            page,
            total_page,
            page_inc: page+1,
            page_dec: if page > 1 {page-1} else {page}
        },
    ))
}
