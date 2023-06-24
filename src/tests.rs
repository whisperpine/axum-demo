// #![allow(unused_imports)]

use super::*;

#[test]
fn demo_test() {
    // assert!(true);
}

#[test]
fn ahash_test() {
    use ahash::AHashMap;
    let mut nice: AHashMap<i32, i32> = AHashMap::new();
    nice.insert(1234, 3);
}

#[test]
#[ignore]
fn buf_lock_write() -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    let output_file = File::create("yahaha.txt")?;
    let mut output = BufWriter::new(output_file);

    let file = File::open("nice.txt")?;
    let mut lock = std::io::stdout().lock();
    for line in BufReader::new(file).lines() {
        let text = line?;
        writeln!(&mut lock, "{}", text)?;
        writeln!(&mut output, "{}", text)?;
    }
    output.flush()?;

    Ok(())
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

#[test]
#[ignore]
fn mem_take() {
    use std::mem;

    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::take(&mut v);
    assert_eq!(vec![1, 2], old_v);
    assert!(v.is_empty());
}
