#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};

use rocket::Request;
use rocket_contrib::Template;
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("./static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(file);

    NamedFile::open(path).ok()
}

// #[derive(Serialize)]
// struct TemplateContext {
//     name: String,
//     items: Vec<String>,
// }

// #[get("/")]
// fn index() -> Template {
//     Template::render("index", {})
// }

// #[get("/hello/<name>")]
// fn get(name: String) -> Template {
//     let context = TemplateContext {
//         name,
//         items: vec![
//             "One",
//             "Two",
//             "Three",
//         ].iter().map(|s| {
//             s.to_string()
//         }).collect(),
//     };

//     Template::render("index", &context)
// }

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files])
        .attach(Template::fairing())
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
