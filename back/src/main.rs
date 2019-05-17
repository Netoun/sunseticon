#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate multipart;

extern crate rocket_cors;
extern crate serde_json;
extern crate sonic_client;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod icon;
mod database;
use std::thread;
use std::time;
use rocket::http::{Method, ContentType, Status};
use rocket::response::NamedFile;

use rocket::Data;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
use multipart::mock::StdoutTee;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};
use icon::Icon;
use sonic_client::search::SearchChan;
use rocket::response::Stream;
use rocket::response::status::Custom;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../vue/dist/index.html")
}

#[get("/<file..>", rank=10)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../vue/dist/").join(file)).ok()
}

#[get("/<file..>", rank = 9)]
fn icons(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/").join(file)).ok()
}

#[get("/list")]
fn list (_connection: database::DbConn) -> JsonValue {
  json!(Icon::get_all(&_connection))
}

#[post("/upload", data = "<data>")]
fn upload (cont_type: &ContentType, data: Data, _connection: database::DbConn ) ->  Result<Stream<Cursor<Vec<u8>>>, Custom<String>>  {
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
    }

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    {
        let stdout = io::stdout();
        let tee = StdoutTee::new(&mut out, &stdout);
        entries.write_debug(tee)?;
    }

    writeln!(out, "Entries processed")
}
pub fn options() -> rocket_cors::Cors {
  rocket_cors::Cors {
    allowed_origins: AllowedOrigins::all(),
    allowed_methods: vec![Method::Post, Method::Get, Method::Put]
      .into_iter()
      .map(From::from)
      .collect(),
    allowed_headers: AllowedHeaders::all(),
    allow_credentials: true,
    ..Default::default()
  }
}

fn rocket() -> rocket::Rocket {
    let conn = database::connect();
    rocket::ignite()
        .manage(conn)
        .attach(options())
        .mount("/", routes![index, files])
        .mount("/icon", routes![icons, upload, list])
}

fn main() {

    // let mut s = SearchChan::new("127.0.0.1", 1491, "SecretPassword").expect("Connection error");
    // let handle = s.read();
    // assert_eq!("CONNECTED <sonic-server v1.2.0>\r\n", s.connect().unwrap());
    // thread::sleep(time::Duration::from_secs(4));
    // let r1 = s
    //     .query("helpdesk", "user:0dcde3a6", "gdpr", Some(50), None)
    //     .unwrap();
    // let r2 = s.ping().unwrap();
    // let r3 = s.quit().unwrap();
    // assert_eq!("EVENT", r1[0]);
    // assert_eq!("PONG\r\n", r2.recv().unwrap());
    // assert_eq!("ENDED quit\r\n", r3.recv().unwrap());
    // handle.join().expect("Failed to wait process");
    
    rocket().launch();
}