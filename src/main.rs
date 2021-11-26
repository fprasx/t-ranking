#[macro_use] extern crate rocket;
extern crate reqwest;
use std::env;
use std::error::Error;

use ranking::aspen;
// Right now all this does is retrieve a session id from aspen for later use
#[get("/")]
async fn index() -> String {
	let session = aspen::get_session().await;
	if let Ok(info) = session {
		println!("Session_id: {}", info.session_id);
		println!("Apache_token: {}", info.apache_token);
		let login = login(&info).await;
		match login {
			Ok(l) => {
				println!("Are we logged in? {}", !l.contains("Invalid login.")); // If the string is not found, then loggedin will be false, so negate it ;
			}
			Err(e) => println!("Error logging in: {}", e),
		}

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
	let username = env::var("ASPEN_USERNAME").unwrap_or_else(|_| String::from("test_username"));
	let password = env::var("ASPEN_PASSWORD").unwrap_or_else(|_| String::from("test_password"));

	let client = reqwest::Client::builder().build()?;
	let res = client
		.post("https://aspen.cpsd.us/aspen/logon.do")
		.header("Cookie", format!("JSESSIONID={}", credentials.session_id))
		// TODO: find some way of more efficiently generating request body!!!
		.query(&["org.apache.struts.taglib.html.TOKEN", credentials.apache_token.as_str()])
		.query(&["userEvent", "930"])
		.query(&["deploymentId", "x2sis"])
		.query(&["username", username.as_str()])
		.query(&["password", password.as_str()])
		// .body(format!(
		// 	"org.apache.struts.taglib.html.TOKEN={}&userEvent=930&deploymentId=x2sis&username={}&password={}",
		// 	credentials.apache_token, username, password
		// ))
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
