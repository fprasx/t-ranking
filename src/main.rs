#[macro_use]
extern crate rocket;
extern crate reqwest;
use reqwest::{header, redirect};
use std::env;
use std::error::Error;

use ranking::aspen::{self, AspenInfo};

// Right now all this does is retrieve a session id from aspen for later use
#[get("/")]
async fn index() -> String {
    if let Ok(info) = aspen::get_session().await {
        println!("Session_id: {}", info.session_id);
        println!("Apache_token: {}", info.apache_token);
        match login(&info).await {
            Ok(l) => {
                println!("Are we logged in? {}", !l.contains("Invalid login."));
                // If the string is not found, then loggedin will be false, so negate it ;
            }
            Err(e) => println!("Error logging in: {}", e),
        }

        match get_student_info(&info).await {
            Ok(student) => student,
            Err(e) => format!("Error: {:#?}", e),
        }
    } else {
        println!("Error Returned");
        "Error Logging in".to_string()
    }
}

async fn login(credentials: &AspenInfo) -> Result<String, Box<dyn Error + Send + Sync>> {
    let username = env::var("ASPEN_USERNAME").unwrap();
    let password = env::var("ASPEN_PASSWORD").unwrap();

    let client = reqwest::Client::builder()
        .redirect(redirect::Policy::none())
        .build()?;

    let params = [
        (
            "org.apache.struts.taglib.html.TOKEN",
            credentials.apache_token.clone(),
        ),
        ("userEvent", "930".to_string()),
        ("deploymentId", "x2sis".to_string()),
        ("username", username.clone()),
        ("password", password.clone()),
    ];

    let res = client
        // .post("https://httpbin.org/post")
        .post("https://aspen.cpsd.us/aspen/logon.do")
        .header(
            header::COOKIE,
            format!("JSESSIONID={}", credentials.session_id.clone()),
        )
        .header(
            header::USER_AGENT,
            "node-fetch/1.0 (+https://github.com/bitinn/node-fetch)",
        )
        .header(header::ACCEPT_ENCODING, "gzip,deflate")
        .form(&params)
        .header(
            header::CONTENT_TYPE,
            "application/x-www-form-urlencoded;charset=UTF-8",
        )
        .send()
        .await?
        .text()
        .await?;
    println!("{}", res);

    /*
    let res = client
        .post("https://aspen.cpsd.us/aspen/logon.do")
        .header("Cookie", format!("JSESSIONID={}", credentials.session_id))
        // TODO: find some way of more efficiently generating request body!!!
        // .query(&["org.apache.struts.taglib.html.TOKEN", credentials.apache_token.as_str()])
        // .query(&["userEvent", "930"])
        // .query(&["deploymentId", "x2sis"])
        // .query(&["username", username.as_str()])
        // .query(&["password", password.as_str()])
        .body(format!(
            "org.apache.struts.taglib.html.TOKEN={}&userEvent=930&deploymentId=x2sis&username={}&password={}",
            credentials.apache_token, username, password
        ))
        .send()
        .await?
        .text()
        .await?;
    */
    Ok("".to_string())
    // Ok(res)
}

async fn get_student_info(
    credentials: &aspen::AspenInfo,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::builder().build()?;
    let res = client
        .post("https://aspen.cpsd.us/aspen/rest/users/students")
        .header("Cookie", format!("JSESSIONID={}", credentials.session_id))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
