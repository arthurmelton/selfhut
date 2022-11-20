#[macro_use]
extern crate rocket;
mod clone;
mod git;
mod index;
mod repository;
mod utils;

use utils::config;

use rocket::fs::relative;

use crate::repository::raw;
use crate::repository::summary;
use crate::repository::tree;
use crate::utils::own_pathbuf::PathBufWithDotfiles;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                file_server,
                index::index,
                summary::repository,
                tree::tree_main,
                tree::tree,
                clone::clone,
                raw::raw
            ],
        )
        .attach(Template::fairing())
}

#[get("/static/<path..>")]
async fn file_server(path: PathBufWithDotfiles) -> Option<rocket::fs::NamedFile> {
    let path = std::path::Path::new(relative!("static")).join(path.get());
    rocket::fs::NamedFile::open(path).await.ok()
}
