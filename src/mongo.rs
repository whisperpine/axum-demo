use crate::error::AppError;
use crate::UserInfo;
use anyhow::Result;
use axum::response::Json;
use mongodb::{options::ClientOptions, Client};
use once_cell::sync::Lazy;

/// Environment variable named `MONGODB_URI`.
const ENV_MONGODB_URI: &str = "MONGODB_URI";
/// Environment variable named `DB_NAME`.
const ENV_DB_NAME: &str = "DB_NAME";

/// Target database name.
static DB_NAME: Lazy<String> = Lazy::new(|| match std::env::var(ENV_DB_NAME) {
    Ok(value) => {
        println!("{}={}", ENV_DB_NAME, value);
        value
    }
    Err(_) => {
        let default_db_name = "axum-demo";
        println!("{} env var hasn't been set", ENV_DB_NAME);
        println!("Using default value: {}", default_db_name);
        default_db_name.to_owned()
    }
});

/// MongoDB Uri.
static MONGODB_URI: Lazy<String> = Lazy::new(|| match std::env::var(ENV_MONGODB_URI) {
    Ok(value) => {
        println!("{}={}", ENV_MONGODB_URI, value);
        value
    }
    Err(_) => {
        let default_uri = "mongodb://localhost:27017";
        println!("{} env var hasn't been set", ENV_MONGODB_URI);
        println!("Using default value: {}", default_uri);
        default_uri.to_owned()
    }
});

/// Read all user info and response.
pub async fn log_mongo() -> Result<Json<Vec<String>>, AppError> {
    let user_infos: Vec<UserInfo> = read_all().await?;
    let mut texts: Vec<String> = vec![];
    for UserInfo { username, id } in user_infos {
        texts.push(format!("{}: {}", username, id));
    }

    Ok(Json(texts))
}

/// Connect to mongodb and get client handle
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
            let error_message = format!("failed to connect to mongodb with in {}", elapsed);
            tracing::error!(error_message);
            anyhow::bail!(error_message)
        }
    };

    // Manually set an option.
    client_options.app_name = Some("axum-demo".to_owned());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    let databases = client.list_database_names(None, None).await?;
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

    for collection_name in db.list_collection_names(None).await? {
        tracing::info!(%collection_name);
    }

    Ok(())
}

/// Insert given [`UserInfo`] to mongodb.
pub async fn insert_userinfo(user_info: &UserInfo) -> Result<()> {
    let db = connect().await?.database(DB_NAME.as_str());
    let collection = db.collection::<UserInfo>("user");
    collection.insert_one(user_info, None).await?;

    Ok(())
}

/// Read all [`UserInfo`] from `DB_NAME` database.
pub async fn read_all() -> Result<Vec<UserInfo>> {
    use futures::TryStreamExt;

    let db = connect().await?.database(DB_NAME.as_str());
    let collection = db.collection::<UserInfo>("user");
    let mut cursor = collection.find(None, None).await?;

    let mut user_infos: Vec<UserInfo> = vec![];
    while let Some(user_info) = cursor.try_next().await? {
        // println!("{:?}", user_info);
        user_infos.push(user_info);
    }

    Ok(user_infos)
}
