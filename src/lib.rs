use axum::{routing::get, Router, response::{IntoResponse, Response, Html}, http::StatusCode, Json};
use sync_wrapper::SyncWrapper;
use askama::Template;

use model::{contact_info::ContactInfo, position::Position, full_resume::ResumeTemplate};

mod model;
pub mod bindings;


struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn contact_info() -> impl IntoResponse {
    let info = ContactInfo {
        name: "Benjamin Peinhardt".to_owned(),
        email: "benjaminpeinhardt@gmail.com".to_owned(),
        phone: "2056410594".to_owned(),
        github_handle: "bcpeinhardt".to_owned(),
    };

    (StatusCode::OK, Json(info))
}

async fn show() -> impl IntoResponse {
    HtmlTemplate(ResumeTemplate {
        name: "Benjamin Peinhardt".to_owned()
    })
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/contact_info", get(contact_info)).route("/show_resume", get(show));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}