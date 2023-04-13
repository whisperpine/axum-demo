// #![allow(unused_imports)]

use super::*;
// use anyhow::Result;

#[test]
fn demo_test() {
    assert!(true);
}

#[test]
fn ahash_test() {
    use ahash::AHashMap;
    let mut nice: AHashMap<i32, i32> = AHashMap::new();
    nice.insert(1234, 3);
}

#[test]
fn buf_lock_write() -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, BufWriter, Write};

    let output_file = File::create("yahaha.txt")?;
    let mut output = BufWriter::new(output_file);

    let file = File::open("nice.txt")?;
    let mut lock = std::io::stdout().lock();
    for line in BufReader::new(file).lines() {
        let text = line?;
        writeln!(lock, "{}", text)?;
        writeln!(output, "{}", text)?;
    }
    output.flush()?;

    Ok(())
}
