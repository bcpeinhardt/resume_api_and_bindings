use axum::{routing::get, Router, response::IntoResponse, http::StatusCode, Json};
use model::contact_info::ContactInfo;
use sync_wrapper::SyncWrapper;

mod model;
pub mod bindings;

async fn contact_info() -> impl IntoResponse {
    let info = ContactInfo {
        name: "Benjamin Peinhardt".to_owned(),
        email: "benjaminpeinhardt@gmail.com".to_owned(),
        phone: "2056410594".to_owned(),
        github_handle: "bcpeinhardt".to_owned(),
    };

    (StatusCode::OK, Json(info))
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/contact_info", get(contact_info));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}