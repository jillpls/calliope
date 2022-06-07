#[macro_use]
extern crate rocket;

extern crate log;

use log::trace;

mod data;
mod parse;
mod posts;
mod users;

use data::database::connect;
use posts::page;
use rocket::fs::{relative, FileServer};

const INSTALLED_VERSION_FILE: &str = "versions";

#[launch]
async fn rocket() -> _ {
    let conn = connect(Some("sqlite.db")).await.unwrap();

    init();

    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/wiki", routes![page])
}

fn init() -> bool {
    let installed_versions = std::fs::read_to_string(INSTALLED_VERSION_FILE);

    if let Ok(v) = installed_versions {
        if v != env!("CARGO_PKG_VERSION") {
            trace!("Version mismatch, starting setup ...");
            println!("Version mismatch, starting setup ...");
            setup();
        }
    } else {
        trace!("No version file found, starting setup ...");
        println!("No version file found, starting setup ...");
        setup();
    }

    return true;
}

fn setup() {
    std::fs::write(INSTALLED_VERSION_FILE, env!("CARGO_PKG_VERSION")).unwrap();
}
