#[macro_use] extern crate rocket;
mod git;
mod index;
mod config;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index])
        .attach(Template::fairing())
}
