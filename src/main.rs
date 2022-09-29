#[macro_use]
extern crate rocket;
mod git;
mod index;
mod repository;
mod utils;

use utils::config;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use crate::repository::summary;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index, summary::repository])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
