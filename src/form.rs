use axum::extract::Form;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    email: String,
}

pub async fn show_form() -> Html<&'static str> {
    Html(include_str!("../pages/form.html"))
}

pub async fn log_form(Form(user_info): Form<UserInfo>) {
    tracing::debug!(?user_info);
}
