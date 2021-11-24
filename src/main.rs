#[macro_use]
extern crate rocket;
extern crate reqwest;
use regex::Regex;

// Request a session id from aspen for later use
async fn get_session_id() -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Make request to aspen using reqwest convenience function
    let res = reqwest::get("https://aspen.cpsd.us/aspen/logon.do")
        .await?
        .text()
        .await?;
    // Regex for finding session id in res
    let re = Regex::new("sessionId='(.+)';").unwrap();
    // Search the response for a match
    let session = re.find(&res[..]);
    match session {
        Some(session) => Ok(Some(session.as_str().to_string())),
        None => Ok(None),
    }
}

// Right now all this does is retrieve a session id from aspen for later use
#[get("/")]
async fn index() -> String {
    let session = get_session_id().await;
    println!("here");
    match session {
        Ok(sesh) => match sesh {
            Some(sesh) => format!("Here is the Session ID Aspen returned: {}", sesh),
            None => format!("No Session ID returned!"),
        },
        Err(e) => format!("Aspen sent an error: {}", e),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

