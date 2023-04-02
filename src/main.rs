mod gen;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Zap! A Superfast E-mail Verification API"
}

#[get("/info")]
fn info() -> &'static str {
    "This is the Info Page for Zap"
    //gen::genotp();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/info",routes![info])
    
}