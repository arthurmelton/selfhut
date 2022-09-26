#[macro_use] extern crate rocket;
mod git;
mod index;
mod config;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use rocket::fs::relative;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
