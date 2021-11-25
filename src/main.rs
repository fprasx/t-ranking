#[macro_use] extern crate rocket;
extern crate reqwest;
use std::error::Error;

use ranking::aspen;

// Right now all this does is retrieve a session id from aspen for later use
#[get("/")]
async fn index() -> String {
	let session = aspen::get_session().await;
	if let Ok(info) = session {
		println!("Session_id: {}", info.session_id);
		println!("Apache_token: {}", info.apache_token);
		let log = login(&info).await.unwrap();

		// TESTING
		println!("Are we logged in? {}", !log.contains("Invalid login.")); // If the string is not found, then loggedin will be false, so negate it
																   // TESTING

		let student = get_student_info(&info).await;
		match student {
			Ok(student) => student,
			Err(e) => format!("Errror: {:#?}", e),
		}
	}
	else {
		println!("Error Returned");
		"Error Logging in".to_string()
	}
}

async fn login(credentials: &aspen::AspenInfo) -> Result<String, Box<dyn Error + Send + Sync>> {
	let client = reqwest::Client::builder().build()?;
	let res = client
		.post("https://aspen.cpsd.us/aspen/logon.do")
		.header("Cookie", format!("JSESSIONID={}", credentials.session_id))
		// TODO: find some way of more efficiently generating request body!!!
		.body(format!(
			"org.apache.struts.taglib.html.TOKEN={}&userEvent=930&deploymentId=x2sis&username={}&password={}",
			credentials.apache_token, "2051549", "test_password"
		))
		.send()
		.await?
		.text()
		.await?;
	Ok(res)
}

async fn get_student_info(credentials: &aspen::AspenInfo) -> Result<String, Box<dyn Error + Send + Sync>> {
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
async fn rocket() -> _ { rocket::build().mount("/", routes![index]) }
