#[macro_use]
extern crate rocket;
extern crate reqwest;
use ranking::aspen::get_aspen;

// Defining route
#[get("/")]
async fn index() -> String {
    match get_aspen().await {
        Ok(res) => {
            res
        }
        Err(e) => {
            e.to_string()
        }
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
