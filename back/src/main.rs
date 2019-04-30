#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate rocket;
extern crate rocket_cors;
extern crate serde;
extern crate serde_json;

mod icon;

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::response::content::Json;
use std::io;
use std::path::{Path, PathBuf};
use icon::Image;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("vue/dist/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("vue/dist/").join(file)).ok()
}

#[derive(Serialize, Debug)]
struct Test {
    title: &'static str
}

#[get("/hello")]
fn hello() -> String {
    let hello_message = Test { title: "Sunseticon" };
    serde_json::to_string(&hello_message).unwrap()
}

#[get("/")]
fn send() -> Json {
   let image = new Image()
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:8000", "http://localhost:8080"]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, files, hello])
        .mount("/icon", routes![send])
        .attach(options)
        .launch();
}