extern crate tera;

use rocket::response::content;
use std::fs::read_to_string;
use std::path::Path;
use tera::{Context, Tera};
use crate::parse;

#[get("/<title>")]
pub fn page(title: &str) -> content::RawHtml<String> {
    let tera = Tera::new("templates/**/*.html").unwrap();
    let mut context = Context::new();
    context.insert("title", &title);
    let path = format!("data/pages/{}", title);
    let content_path = Path::new(&path);
    if !content_path.exists() {
        content::RawHtml(tera.render("page_not_found.html", &context).unwrap())
    } else {
        let content = read_to_string(content_path).unwrap();
        let content = parse::parse_content(&content, parse::ParseOptions::None, Box::from(parse::parse_markdown));
        context.insert("content", &content);
        content::RawHtml(tera.render("post.html", &context).unwrap())
    }
}
