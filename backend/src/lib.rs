use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sync_wrapper::SyncWrapper;

use model::contact_info::ContactInfo;

pub mod bindings;
mod model;

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
    let router = Router::new()
        .merge(axum_extra::routing::SpaRouter::new("/assets", "dist"))
        .route("/contact_info", get(contact_info));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
