#[macro_use]
extern crate rocket;
mod git;
mod index;
mod repository;
mod utils;
mod clone;

use utils::config;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use crate::repository::summary;
use crate::repository::tree;
use crate::repository::raw;
use crate::utils::own_pathbuf::PathBufWithDotfiles;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![file_server, index::index, summary::repository, tree::tree_main, tree::tree, clone::clone, raw::raw])
        .attach(Template::fairing())
}

#[get("/static/<path..>")]
async fn file_server(path: PathBufWithDotfiles) -> Option<rocket::fs::NamedFile> {
    let path = std::path::Path::new(relative!("static")).join(path.get());
    rocket::fs::NamedFile::open(path).await.ok()
}
