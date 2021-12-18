#[macro_use]
extern crate rocket;
extern crate reqwest;
use ranking::aspen::{extract_classes, get_classes};

#[get("/")]
async fn index() -> String {
    match get_classes().await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

#[get("/classes")]
async fn classes() -> String {
    match get_classes().await {
        Ok(res) => match extract_classes(res) {
            Ok(classes) => classes,
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![classes])
}
