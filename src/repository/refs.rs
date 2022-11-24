use crate::config::CONFIG;
use crate::git::branches::branches;


use crate::git::tag::get_tag;
use crate::utils::repo_config::repo_config;
use rocket_dyn_templates::{context, Template};

#[get("/<repo>/refs?<page>", rank = 2)]
pub fn refs(repo: String, page: Option<usize>) -> Option<Template> {
    let mut tags = get_tag(repo.clone(), 10, (page.unwrap_or(1)-1)*10, None);
    let mut pages = 1;
    match tags {
        Some(ref mut x) => pages = x.1/10+1,
        None => {}
    }
    Some(Template::render(
        "repository/refs",
        context! {
            title: format!("/ :: {}", repo.clone()),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            active: "refs",
            current_dir_file: "/",
            current_dir: "/",
            payment: CONFIG.payment_link.clone(),
            mailing_list: CONFIG.mailing_list.clone(),
            branch: branches(repo.clone()),
            tag: tags,
            page_dec: page.unwrap_or(1)-1,
            page_inc: page.unwrap_or(1)+1,
            total_page: pages,
            page: page.unwrap_or(1)
        },
    ))
}

#[get("/<repo>/refs/<name>", rank = 2)]
pub fn refs_id(repo: String, name: String) -> Option<Template> {
    let binding = get_tag(repo.clone(), 1, 0, Some(name))?;
    let tag = binding.0.first()?;
    Some(Template::render(
        "repository/ref",
        context! {
            title: format!("/ :: {}", repo.clone()),
            repo: repo.clone(),
            config: repo_config(repo.clone()),
            domain: CONFIG.domain.to_string(),
            active: "refs",
            current_dir_file: "/",
            current_dir: "/",
            payment: CONFIG.payment_link.clone(),
            mailing_list: CONFIG.mailing_list.clone(),
            tag: tag,
        },
    ))
}
