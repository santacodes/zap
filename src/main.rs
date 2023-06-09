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

    email: String,
    passkey: String

}

#[post("/", format = "json", data = "<user_input>")]
fn hellopost(user_input: Json<User>) -> String {
    let x = format!("print test {:?} \n", user_input);
    println!("{}",x);
    return x;
}

fn main() {
    let _myconnection = database::connectiontopostgres();
    rocket::ignite()
    .mount("/hello", routes![hellopost])
    .mount("/", routes![index])
    .launch();
    
    
}   
