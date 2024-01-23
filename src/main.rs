#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_json;

use rocket::Data;
use rocket::response::content;
use serde_json::Value;
use std::io::Read; // Add import for Read trait
use rocket::response::NamedFile;
use std::path::PathBuf;

fn try_fix_json(json: &str) -> String {
    json.replace("'", "\"") // replace single quotes with double quotes
        .replace(",}", "}") // remove trailing commas in objects
        .replace(",]", "]") // remove trailing commas in arrays
}

#[post("/prettify", data = "<data>")]
fn prettify(data: Data) -> content::Json<String> {
    let mut json = String::new();
    if let Err(error) = data.open().read_to_string(&mut json) {
        return content::Json(format!("Error reading data: {}", error));
    }
    json = try_fix_json(&json);
    let v: Value = match serde_json::from_str(&json) {
        Ok(value) => value,
        Err(error) => return content::Json(format!("Error parsing JSON: {}", error)),
    };
    content::Json(serde_json::to_string_pretty(&v).unwrap())
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(PathBuf::from("src/index.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, prettify])
        .launch();
}