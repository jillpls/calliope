extern crate tera;
extern crate uuid;

use crate::data::database::{get_login_info, insert_user, update_user_session};
use crate::users::passwords::{hash_password, verify_password};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content;
use tera::{Context, Tera};
use uuid::{uuid, Uuid};

#[derive(FromForm)]
pub struct NewUser {
    login: String,
    password: String,
    repeat: String,
}

#[derive(FromForm)]
pub struct LoginUser {
    login: String,
    password: String,
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

#[get("/login")]
pub async fn login_user_get() -> content::RawHtml<String> {
    let tera = Tera::new("templates/**/*.html").unwrap();
    content::RawHtml(tera.render("login.html", &Context::new()).unwrap())
}

#[post("/login", data = "<user>")]
pub async fn login_user(
    user: Form<LoginUser>,
    cookies: &CookieJar<'_>,
) -> content::RawHtml<String> {
    let tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = Context::new();
    let login_info = get_login_info(&user.login, None).await;
    match login_info {
        Err(_) => {
            let errors: String = "Benutzername oder Passwort falsch".to_string();
            context.insert("errors", &errors);
        }
        Ok(result) => {
            if !result.activated {
                let errors: String = "Account ist nicht aktiviert.".to_string();
                context.insert("errors", &errors);
            } else {
                if verify_password(user.password.as_bytes(), &result.hash) {
                    let session_id = Uuid::new_v4();
                    match update_user_session(&result.id, &session_id.to_string(), None).await {
                        Ok(_) => {
                            cookies.add_private(Cookie::new("user_id", result.id.to_string()));
                            cookies.add_private(Cookie::new("session", session_id.to_string()));
                            context.insert("success", &true);
                        }
                        Err(_) => {
                            let errors: String = "Konnte Login nicht anlegen".to_string();
                            context.insert("errors", &errors);
                        }
                    }
                } else {
                    let errors: String = "Benutzername oder Passwort falscht".to_string();
                    context.insert("errors", &errors);
                }
            }
        }
    }
    content::RawHtml(tera.render("login.html", &context).unwrap())
}
