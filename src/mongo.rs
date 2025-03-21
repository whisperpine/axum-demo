use crate::AppError;
use anyhow::Result;
use axum::extract::Form;
use axum::response::Json;
use mongodb::{bson::Document, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use uuid::Uuid;

/// Environment variable named `MONGODB_URI`.
const ENV_MONGODB_URI: &str = "MONGODB_URI";
/// Environment variable named `DB_NAME`.
const ENV_DB_NAME: &str = "DB_NAME";

/// Target database name.
static DB_NAME: LazyLock<String> = LazyLock::new(|| match std::env::var(ENV_DB_NAME) {
    Ok(value) => {
        tracing::info!("{}={}", ENV_DB_NAME, value);
        value
    }
    Err(_) => {
        let default_db_name = "axum-demo";
        tracing::warn!("{} env var hasn't been set", ENV_DB_NAME);
        tracing::warn!("Using default value: {}", default_db_name);
        default_db_name.to_owned()
    }
});

/// MongoDB connection string.
static MONGODB_URI: LazyLock<String> = LazyLock::new(|| match std::env::var(ENV_MONGODB_URI) {
    Ok(value) => {
        tracing::info!("{}={}", ENV_MONGODB_URI, value);
        value
    }
    Err(_) => {
        let default_uri = "mongodb://localhost:27017";
        tracing::warn!("{} env var hasn't been set", ENV_MONGODB_URI);
        tracing::warn!("Using default value: {}", default_uri);
        default_uri.to_owned()
    }
});

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    username: String,
    id: Uuid,
}

/// Read all user info and response.
pub async fn log_registered_users() -> Result<Json<Vec<String>>, AppError> {
    let user_infos: Vec<UserInfo> = read_all().await?;
    let mut texts: Vec<String> = vec![];
    for UserInfo { username, id } in user_infos {
        texts.push(format!("{username}: {id}"));
    }
    Ok(Json(texts))
}

/// Connect to mongodb and get client handle.
async fn connect() -> Result<Client> {
    use std::time::Duration;
    use tokio::time::timeout;

    // Parse a connection string into an options struct.
    let mut client_options = match timeout(
        Duration::from_millis(200),
        ClientOptions::parse(MONGODB_URI.as_str()),
    )
    .await
    {
        Ok(inner) => inner?,
        Err(elapsed) => {
            let error_message = format!("failed to connect to mongodb with in {elapsed}");
            tracing::error!(error_message);
            anyhow::bail!(error_message)
        }
    };

    // Manually set an option.
    client_options
        .app_name
        .get_or_insert(crate::CRATE_NAME.to_owned());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    let databases = client.list_database_names().await?;
    tracing::debug!(?databases);

    Ok(client)
}

/// List all collections in determined database.
/// # Example
/// ```
/// # async fn demo() {
/// // "admin" is one of the default database in mongodb
/// axum_demo::mongo::list_collections("admin").await.unwrap();
/// # }
/// ```
pub async fn list_collections(db_name: &str) -> Result<()> {
    let db = connect().await?.database(db_name);
    for collection_name in db.list_collection_names().await? {
        tracing::info!(%collection_name);
    }
    Ok(())
}

/// Insert given [`UserInfo`] to mongodb.
async fn insert_userinfo(user_info: &UserInfo) -> Result<()> {
    let db = connect().await?.database(DB_NAME.as_str());
    let collection = db.collection::<UserInfo>("user");
    collection.insert_one(user_info).await?;

    Ok(())
}

/// Read all [`UserInfo`] from `DB_NAME` database.
async fn read_all() -> Result<Vec<UserInfo>> {
    use futures::TryStreamExt;

    let db = connect().await?.database(DB_NAME.as_str());
    let collection = db.collection::<UserInfo>("user");
    let mut cursor = collection.find(Document::new()).await?;

    let mut user_infos: Vec<UserInfo> = vec![];
    while let Some(user_info) = cursor.try_next().await? {
        // println!("{:?}", user_info);
        user_infos.push(user_info);
    }

    Ok(user_infos)
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    username: String,
}

/// Add [`UserInfo`] to database and response in json format.
pub async fn register_user(Form(value): Form<CreateUser>) -> Result<Json<UserInfo>, AppError> {
    let user_info = UserInfo {
        username: value.username,
        id: Uuid::new_v4(),
    };
    insert_userinfo(&user_info).await?;
    Ok(Json(user_info))
}
