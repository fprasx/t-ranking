use std::{error::Error, fmt};

use regex::Regex;
use reqwest;

#[derive(Clone)]
pub struct AspenInfo {
    pub session_id: String,
    pub apache_token: String,
}

impl AspenInfo {
    fn new(session_id: String, apache_token: String) -> AspenInfo {
        AspenInfo {
            session_id,
            apache_token,
        }
    }
}

#[derive(Debug)]
pub enum AspenError {
    NoSession,
}

// Placeholder implementation
impl fmt::Display for AspenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // As more error types are created this should basically become a case statement
        match self {
            AspenError::NoSession => write!(f, "NoSession Error, Invalid Response Returned"),
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
pub async fn get_session() -> Result<AspenInfo, Box<dyn Error + Send + Sync>> {
    // Make request to aspen using reqwest convenience function
    let res = reqwest::get("https://aspen.cpsd.us/aspen/logon.do")
        .await?
        .text()
        .await?;
    // Regex for finding session id in res (regex from https://github.com/Aspine/aspine/blob/master/src/scrape.ts:762)
    let session_re = Regex::new("sessionId='(.+)';").unwrap();
    // Regex for finding apache token in res (regex from https://github.com/Aspine/aspine/blob/master/src/scrape.ts:766)
    let apache_token_re =
        Regex::new("name=\"org.apache.struts.taglib.html.TOKEN\" value=\"(.+)\"").unwrap();
    // Look at regex documentation for match groups
    let session = session_re.find(&res[..]); // EX: sessionId='2llmtAkaAwPnICzAVc_2qeK2RhRzpcVhdB4vhGbB';
    let token = apache_token_re.find(&res[..]); // EX: name="org.apache.struts.taglib.html.TOKEN"
                                                // value="843ad705c44d2f6cadf8b454db87fc39" If neither a session_id or
                                                // apache_token is found, return an AspenError
    if session.is_none() || token.is_none() {
        return Err(Box::new(AspenError::NoSession));
    }
    // Unwrapping regex match and taking substrings to get just values
    let session_ret = session
        .unwrap()
        .as_str()
        .chars()
        .skip(11)
        .take(40)
        .collect::<String>();
    let token_ret = token
        .unwrap()
        .as_str()
        .chars()
        .skip(50)
        .take(32)
        .collect::<String>();
    Ok(AspenInfo::new(session_ret, token_ret))
}
