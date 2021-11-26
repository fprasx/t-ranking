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
    // Check that both regex's mactched
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
