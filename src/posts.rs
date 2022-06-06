use askama::Template;
use rocket::response::content;

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    title: &'a str,
}


#[get("/<title>")]
pub fn page(title: &str) -> content::RawHtml<String> {
    let page = PostTemplate { title : title };
    content::RawHtml(page.render().unwrap())
}