use regex::Regex;
use reqwest;
use reqwest::header;
use std::env;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct AspenInfo {
    pub session_id: String,
    pub apache_token: String,
}

impl AspenInfo {
    // change to private later
    pub fn new(session_id: String, apache_token: String) -> AspenInfo {
        AspenInfo {
            session_id,
            apache_token,
        }
    }
}

#[derive(Debug)]
pub enum AspenError {
    NoSession,
    InvalidLogin,
}

// Placeholder implementation
impl fmt::Display for AspenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // As more error types are created this should basically become a case statement
        match self {
            AspenError::NoSession => write!(f, "NoSession Error, Invalid Response Returned"),
            AspenError::InvalidLogin => write!(f, "InvalidLogin Error, Please Try Again"),
        }
    }
}

// Placeholder implementation
impl Error for AspenError {
    fn description(&self) -> &str {
        "AspenError"
    }
}

// Request a session id from aspen for later use
async fn get_session() -> Result<AspenInfo, Box<dyn Error + Send + Sync>> {
    // Make request to aspen using reqwest convenience function
    let res = reqwest::get("https://aspen.cpsd.us/aspen/logon.do")
        .await?
        .text()
        .await?;
    // Regex's from https://github.com/Aspine/aspine/blob/master/src/scrape.ts:762)
    let session_re = Regex::new("sessionId='(.+)';").unwrap();
    let apache_token_re =
        Regex::new("name=\"org.apache.struts.taglib.html.TOKEN\" value=\"(.+)\"").unwrap();
    // EX: sessionId='2llmtAkaAwPnICzAVc_2qeK2RhRzpcVhdB4vhGbB';
    let session = session_re.captures(res.as_str());
    // EX: name="org.apache.struts.taglib.html.TOKEN" value="843ad705c44d2f6cadf8b454db87fc39"
    let token = apache_token_re.captures(res.as_str());
    // If one either the session_id or the apache token isn't found, return an error
    if session.is_none() || token.is_none() {
        return Err(Box::new(AspenError::NoSession));
    }
    // Check that both regex's matched
    if let (Some(s_inner), Some(t_inner)) = (session, token) {
        // Check that the groups were found
        if let (Some(s), Some(t)) = (s_inner.get(1), t_inner.get(1)) {
            Ok(AspenInfo::new(
                s.as_str().to_string(),
                t.as_str().to_string(),
            ))
        } else {
            Err(Box::new(AspenError::NoSession))
        }
    } else {
        Err(Box::new(AspenError::NoSession))
    }
}

pub async fn get_aspen() -> Result<String, Box<dyn Error + Send + Sync>> {
    // In future, get user credentials from frontend
    let username = env::var("ASPEN_USERNAME").unwrap();
    let password = env::var("ASPEN_PASSWORD").unwrap();
    let client = reqwest::Client::builder().build()?;
    // Getting session_id and apache_token from Aspen
    let info = get_session().await?;
    // Login to aspen
    let login_res = client
        .post("https://aspen.cpsd.us/aspen/logon.do")
        .header(
            header::COOKIE,
            format!("JSESSIONID={}.aspen-app2", info.session_id),
        )
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36")
        .query(&[("org.apache.struts.taglib.html.TOKEN", info.apache_token)])
        .query(&[("userEvent", "930")])
        .query(&[("deploymentId", "x2sis")])
        .query(&[("username", username)])
        .query(&[("password", password)])
        .send()
        .await?
        .text()
        .await?;
    // Check if login was successful
    if login_res.contains("Invalid login.") {
        return Err(Box::new(AspenError::InvalidLogin));
    }
    // TODO: see aspine's get_academics() and get_class_details() in src/scrape.ts
    // Sample request, getting list of classes
    let res = client
        .get("https://aspen.cpsd.us/aspen/portalClassList.do?navkey=academics.classes.list")
        .header("Cookie", format!("JSESSIONID={}", info.session_id))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
