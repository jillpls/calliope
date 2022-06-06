#[macro_use] extern crate rocket;

mod posts;

use rocket::fs::{FileServer, relative};
use posts::page;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from(relative!("static"))).mount("/wiki", routes![page])
}