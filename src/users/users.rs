extern crate tera;

use crate::data::database::insert_user;
use crate::users::passwords::hash_password;
use rocket::form::Form;
use rocket::response::content;
use tera::{Context, Tera};

#[derive(FromForm)]
pub struct NewUser {
    login: String,
    password: String,
    repeat: String,
}

#[get("/register")]
pub async fn register_user_get() -> content::RawHtml<String> {
    let tera = Tera::new("templates/**/*.html").unwrap();
    content::RawHtml(tera.render("register.html", &Context::new()).unwrap())
}

#[post("/register", data = "<user>")]
pub async fn register_user(user: Form<NewUser>) -> content::RawHtml<String> {
    let tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = Context::new();
    if user.password == user.repeat {
        let hash = hash_password(user.password.as_bytes(), None, None)
            .unwrap()
            .to_string();
        match insert_user(&user.login, &hash, None).await {
            Ok(_) => {
                context.insert("success", &true);
            }
            Err(e) => {
                let errors: String = "Loginname bereits vergeben.".to_string();
                context.insert("errors", &errors);
            }
        }
    } else {
        let errors: String = "Passwörter stimmen nicht überein".to_string();
        context.insert("errors", &errors);
    }
    content::RawHtml(tera.render("register.html", &context).unwrap())
}
