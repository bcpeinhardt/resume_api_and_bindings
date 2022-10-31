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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contact_info() {

        let info = ContactInfo {
            name: "Benjamin Peinhardt".to_owned(),
            email: "benjaminpeinhardt@gmail.com".to_owned(),
            phone: "2056410594".to_owned(),
            github_handle: "bcpeinhardt".to_owned(),
        };

        assert_eq!(contact_info().await.unwrap(), info);
    }
}