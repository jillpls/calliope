#[macro_use]
extern crate rocket;

extern crate log;

use log::trace;

mod data;
mod parse;
mod posts;
mod users;

use data::database::{connect, UserData};
use rocket_db_pools::Database;
use posts::page;
use rocket::fs::{relative, FileServer};
use sqlx;

const INSTALLED_VERSION_FILE: &str = "versions";

#[launch]
async fn rocket() -> _ {
    init().await;

    rocket::build()
    .attach(UserData::init())
        .mount(
            "/users",
            routes![
                users::register_user,
                users::register_user_get,
                users::login_user_get,
                users::login_user
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/wiki", routes![page])
}

async fn init() -> bool {
    let installed_versions = std::fs::read_to_string(INSTALLED_VERSION_FILE);

    if let Ok(v) = installed_versions {
        if v != env!("CARGO_PKG_VERSION") {
            trace!("Version mismatch, starting setup ...");
            println!("Version mismatch, starting setup ...");
            setup().await;
        }
    } else {
        trace!("No version file found, starting setup ...");
        println!("No version file found, starting setup ...");
        setup().await;
    }

    return true;
}

async fn setup() {
    let mut conn = connect(Some("sqlite.db")).await.unwrap();
    println!("Established connection");
    sqlx::query(
        "
create table users(
    id int auto_increment primary key,
    login varchar(30) not null unique,
    hash varchar(255) not null,
    activated int not null,
    session char(36),
    email varchar(30)
)
",
    )
    .execute(&mut conn)
    .await
    .expect("Failed creating table");

    std::fs::write(INSTALLED_VERSION_FILE, env!("CARGO_PKG_VERSION")).unwrap();
}
