#[macro_use]
extern crate rocket;
mod config;
mod git;
mod index;

use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
