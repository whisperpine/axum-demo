use axum::extract::Form;
use axum::response::Html;
use serde::Deserialize;

#[expect(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    email: String,
}

/// Response with a html page which contains a form.
pub async fn show_form() -> Html<&'static str> {
    Html(include_str!("../pages/form.html"))
}

/// Log what's been submitted in the form.
pub async fn log_form(Form(user_info): Form<UserInfo>) {
    tracing::debug!(?user_info);
}
