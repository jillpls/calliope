#[macro_use]
extern crate rocket;

mod data;
mod parse;
mod posts;
mod users;

use data::database::connect;
use posts::page;
use rocket::fs::{relative, FileServer};

#[launch]
async fn rocket() -> _ {
    let conn = connect(Some("sqlite.db")).await.unwrap();

    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/wiki", routes![page])
}
