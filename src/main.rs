#[macro_use] extern crate rocket;
use std::env;

#[get("/")]
fn index() -> String {
	let name = env::var("USER").unwrap_or(String::from("Couldn't find hostname"));
	return  format!("Hello, {}", name)
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
