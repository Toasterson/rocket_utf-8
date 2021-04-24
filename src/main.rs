#![feature(decl_macro)]

#[macro_use]
use rocket::{get, routes};

use rocket::http::RawStr;

#[get("/hello?<filename>")]
fn hello(filename: &RawStr) -> String {
    std::fs::read_to_string(filename.url_decode().unwrap() + ".txt").unwrap()
}

fn main() {
    rocket::ignite().mount("/base", routes![hello]).launch();
}
