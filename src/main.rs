mod gen;
mod email;
mod database;

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Zap! A Superfast E-mail Verification API"
}

#[get("/info")]
fn info() -> &'static str {
    "This is the Info Page for Zap" 
}
//Check the validity of email before
#[get("/hello/<email>/<valid>")]
fn hello(email: &str, valid: bool) -> String {
    database::establish_connection();
    if valid {
        let x = gen::genotp();
        format!("Your email is  {}! and your generated otp is {}", email, x)
    } else {
        format!("Your email was not valid")
    }
}
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct User {
    email: String,
    password: String,
}

#[post("/", format = "json", data = "<user_input>")]
fn helloPost(user_input: Json<User>) -> String {
    format!("print test {:?}", user_input)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/info",routes![info])
    .mount("/",routes![hello])
    .mount("/verification", routes![helloPost]).launch();
    
}