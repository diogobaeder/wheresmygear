#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();

    context.insert("foo", "bar-again");

    Template::render("index", &context)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files]).attach(
        Template::fairing(),
    )
}

fn main() {
    rocket().launch();
}
