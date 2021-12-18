#[macro_use]
extern crate rocket;
extern crate reqwest;
use ranking::aspen::{get_aspen, get_classes};

#[get("/")]
async fn index() -> String {
    match get_aspen().await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

#[get("/classes")]
async fn classes() -> String {
    // match get_classes(String::new()) {
    //     Ok(res) => res,
    //     Err(e) => e.to_string()
    // }
    match get_aspen().await {
        Ok(res) => match get_classes(res) {
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
