// #![allow(unused_imports)]

use super::*;

#[test]
fn ahash_test() {
    use ahash::AHashMap;
    let mut nice: AHashMap<i32, i32> = AHashMap::new();
    nice.insert(1234, 3);
}

#[tokio::test]
async fn insert_userinfo_test() -> Result<()> {
    let user_info = UserInfo {
        username: "test name".to_owned(),
        id: uuid::Uuid::new_v4(),
    };
    mongo::insert_userinfo(&user_info).await?;

    Ok(())
}

#[tokio::test]
async fn read_all_document_test() -> Result<()> {
    mongo::read_all().await?;
    Ok(())
}
