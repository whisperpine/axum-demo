use axum::extract::Form;
use axum::response::Html;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    email: String,
}

pub async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">

        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>axum demo</title>
        </head>

        <body>
            <h1>form</h1>

            <form method="post">
                <label for="id_username"> username: </label>
                <input type="text" name="username" id="id_username">

                <label for="id_email"> email: </label>
                <input type="text" name="email" id="id_email">

                <button type="submit">submit</button>
            </form>
        </body>

        </html>
   "#,
    )
}

pub async fn log_form(Form(user_info): Form<UserInfo>) {
    dbg!(user_info);
}
