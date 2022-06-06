use askama::Template;
use rocket::response::content;
use std::path::Path;
use std::fs::read_to_string;

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Template)]
#[template(path = "page_not_found.html")]
struct PageNotFoundTemplate<'a> {
    title: &'a str,
}

#[get("/<title>")]
pub fn page(title: &str) -> content::RawHtml<String> {
    let path = format!("data/pages/{}", title);
    let content_path = Path::new(&path);
    if ! content_path.exists() {
        content::RawHtml( (PageNotFoundTemplate { title : title }).render().unwrap() )
    } else {
        let content = read_to_string(content_path).unwrap(); 
        let page = PostTemplate { title : title, content : &content };
        content::RawHtml(page.render().unwrap())
    }
}