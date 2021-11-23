#[macro_use] extern crate rocket;
use std::env;

#[get("/")]
fn index() -> String {
	// Gets USER env variable
	let name = env::var("USER",).unwrap_or_else(|_| String::from("Couldn't find hostname",),);
	return format!("Hello, {}", name);
}

#[launch]
fn rocket() -> _ { rocket::build().mount("/", routes![index],) }
