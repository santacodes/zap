#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Zap! A Superfast E-mail Verification API"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}