//! Rust bindings to the REST api, built on a simple Request client.

use crate::model::contact_info::ContactInfo;

pub type ResumeApiResult<T> = Result<T, reqwest::Error>;

pub async fn contact_info() -> ResumeApiResult<ContactInfo> {
    let resp = reqwest::get("https://hello_world.shuttleapp.rs/hello")
        .await?
        .json()
        .await?;
    Ok(resp)
}