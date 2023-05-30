mod gen;
mod email;
mod database;

#[macro_use] extern crate rocket;

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


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/info",routes![info])
    .mount("/",routes![hello])
    
}