//In rust 2018, we almostly don't need extern crate anymore.
#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_contrib::templates::Template;

mod fetch;
use crate::fetch::*;

const RSS_URL: &str = "https://news.ycombinator.com/rss";

#[get("/")]
fn index() -> Template {
    let news = fetch_from(RSS_URL).ok().expect("Could not read RSS");
    Template::render("index", &news)
}

fn main() {
    rocket::ignite().
        mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
