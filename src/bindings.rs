//! Rust bindings to the REST api, built on a simple Request client.

use reqwest::Client;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::model::contact_info::ContactInfo;

#[derive(Error, Debug)]
pub enum ResumeApiError {
    
    #[error("Error performing request")]
    ReqwestErr,

    #[error("Error parsing response")]
    ParseErr
}

pub type ResumeApiResult<T> = Result<T, ResumeApiError>;

fn client() -> Client {
    Client::new()
}

pub async fn contact_info() -> ResumeApiResult<ContactInfo> {
    let resp = reqwest::get("https://ben_peinhardt_resume_api.shuttleapp.rs/contact_info")
        .await.map_err(|_| ResumeApiError::ReqwestErr)?
        .json()
        .await.map_err(|_| ResumeApiError::ParseErr)?;
    Ok(resp)
}

