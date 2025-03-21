// #![allow(unused_imports)]

// use super::*;

#[test]
fn ahash_test() {
    use ahash::AHashMap;
    let mut nice: AHashMap<i32, i32> = AHashMap::new();
    nice.insert(1234, 3);
}
