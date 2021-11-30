use std::{error::Error, fmt};

use regex::Regex;
use reqwest::{self, Client};
use thiserror::Error;

#[derive(Clone)]
pub struct AspenInfo {
    client: Client,

    pub session_id: String,
    pub apache_token: String,
}

impl AspenInfo {
    pub async fn new() -> Result<AspenInfo, ProjError> {
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
                .ok_or(ProjError::from(AspenError::NoSession))?
                .get(1)
                .ok_or(ProjError::from(AspenError::NoSession))?
                .as_str()
                .to_owned()
        }
        Ok(ret)
    }
}

#[derive(Error, Debug)]
pub enum ProjError {
    #[error("AspenError")]
    Aspen(#[from] AspenError),
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum AspenError {
    #[error("NoSession Error, Invalid Response Returned")]
    NoSession,
}
