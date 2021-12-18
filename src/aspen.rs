use regex::Regex;
use reqwest::header;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use thiserror::Error;

// Holds info for one class
#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub teacher: String,
    pub room: usize,
    pub class: String,
}

#[derive(Clone, Debug)]
pub struct AspenInfo {
    client: Client,
    pub session_id: String,
    pub apache_token: String,
}

impl AspenInfo {
    async fn new() -> Result<AspenInfo, ProjError> {
        let client = reqwest::Client::new();
        let [session_id, apache_token] = AspenInfo::get_session(&client).await?;
        Ok(AspenInfo {
            client,
            session_id,
            apache_token,
        })
    }

    // Request a session id from aspen for later use [session_id, apache_token]
    async fn get_session(client: &Client) -> Result<[String; 2], ProjError> {
        let res = client
            .get("https://aspen.cpsd.us/aspen/logon.do")
            .send()
            .await?
            .text()
            .await?;
        let mut ret = [String::default(), String::default()];
        for (i, pattern) in [
            "sessionId='(.+)';", // Regex for finding session id in res (regex from https://github.com/Aspine/aspine/blob/master/src/scrape.ts:762)
            "name=\"org.apache.struts.taglib.html.TOKEN\" value=\"(.+)\"", // Regex for finding apache token in res (regex from https://github.com/Aspine/aspine/blob/master/src/scrape.ts:766)
        ]
        .iter()
        .enumerate()
        {
            ret[i] = Regex::new(pattern)
                .unwrap()
                .captures(&res)
                .ok_or_else(|| ProjError::from(AspenError::NoSession))?
                .get(1)
                .ok_or_else(|| ProjError::from(AspenError::NoSession))?
                .as_str()
                .to_owned()
        }
        Ok(ret)
    }

    // Can take ownership of self because this should be the last method called
    async fn logout(self) -> Result<(), ProjError> {
        let client = self.client;
        client
            .post("https://aspen.cpsd.us/aspen/logout.do")
            .header(
                header::COOKIE,
                format!("JSESSIONID={}.aspen-app2", self.session_id),
            )
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ProjError {
    #[error("AspenError")]
    Aspen(#[from] AspenError),
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
    #[error("serde_json error: {0}")]
    JSONError(#[from] serde_json::error::Error),
}

#[derive(Error, Debug)]
pub enum AspenError {
    #[error("NoSession Error, Invalid Response Returned")]
    NoSession,
    #[error("InvalidLogin Error, Please Try Again")]
    InvalidLogin,
}

pub async fn get_aspen() -> Result<String, ProjError> {
    // In future, get user credentials from frontend
    let username = env::var("ASPEN_USERNAME").unwrap();
    let password = env::var("ASPEN_PASSWORD").unwrap();
    // Getting session_id and apache_token from Aspen
    let info = AspenInfo::new().await?;
    let client = &info.client;
    // Login to aspen
    let login_res = client
        .post("https://aspen.cpsd.us/aspen/logon.do")
        .header(
            header::COOKIE,
            format!("JSESSIONID={}.aspen-app2", info.session_id.clone()),
        )
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36")
        .query(&[("org.apache.struts.taglib.html.TOKEN", info.apache_token.clone())])
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
        return Err(ProjError::Aspen(AspenError::InvalidLogin));
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
    // Logout/close session
    info.logout().await?;
    Ok(res)
}

// Takes in response from aspen
// Uses regex to find class info: class name, room number, and teacher
// Returns JSON string with class list
pub fn get_classes(aspenres: String) -> Result<String, ProjError> {
    // Capture group 1: the teacher's name in the format Last, First
    // Capture group 2: Room number
    // Capture group 3: Class name
    let re = Regex::new(r"<td nowrap>\s*([A-Z]{1}[a-zA-Z-]+, [A-Z]{1}[a-zA-Z-]+)\s*</td>\s*<td nowrap>\s*([\d]+)\s*</td>\s*<td nowrap>\s*([a-zA-z: ]+)\s*</td>").unwrap();
    let caps = re.captures_iter(&aspenres);
    let mut info = Vec::<Class>::new();
    // Functional programming is cool
    caps.for_each(|cap| info.push(Class {
        // Remove newlines
        teacher: cap.get(1).unwrap().as_str().to_string().replace("\n", " "),
        // Turn string to usize
        room: cap
            .get(2)
            .unwrap()
            .as_str()
            .to_string()
            .replace("\n", " ")
            .parse::<usize>()
            .unwrap(),
        // Due to the regex, there may be repeated spaces within the class name as well as new lines and tabs which should be removed
        // TODO: figure out bettwe way to remove whitespace between words in class name
        class: cap
            .get(3)
            .unwrap()
            .as_str()
            .to_string()
            .replace("\n", " ")
            .replace("\t", " ")
            .replace("  ", ""),
    }));

    // Return class list as JSON
    // If no classes found, will return []
    let json = serde_json::to_string(&info)?;
    Ok(json)
}
