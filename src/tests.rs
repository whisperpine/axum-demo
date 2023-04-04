#![allow(unused_imports)]

use super::*;

#[test]
fn demo_test() {
    assert!(true);
}

#[test]
fn ahash_test() {
    use ahash::{AHashMap, RandomState};
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
    map.insert(12, 34);

    let mut nice: AHashMap<i32, i32> = AHashMap::new();
    nice.insert(1234, 3);
}
