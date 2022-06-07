#[macro_use]
extern crate rocket;

mod parse;
mod posts;
mod users;

use posts::page;
use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/wiki", routes![page])
}
