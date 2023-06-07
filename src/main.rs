#![feature(decl_macro)]
mod gen;
mod email;
mod database;

use rocket_contrib::json::Json;
use serde::Deserialize;
#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Welcome to Zap! A Superfast E-mail Verification API"
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct User {
    id: i64,
    email: String,
    password: String,
    otp: i64
}

#[post("/", format = "json", data = "<user_input>")]
fn hellopost(user_input: Json<User>) -> String {
    format!("print test {:?}", user_input)
}

fn main() {
    rocket::ignite()
    .mount("/hello", routes![hellopost])
    .mount("/", routes![index])
    .launch();

}   
