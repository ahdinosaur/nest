// TODO test .get when path is empty
// TODO test .get when path is 
// TODO test .set when path is deeper than existing value

extern crate assert_fs;
extern crate predicates;

use assert_fs::prelude::*;
use predicates::prelude::*;
use serde_json::json;

use nest;

mod common;

#[test]
fn get_simple () {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp
        .child("hello/world.json")
        .write_str(r#"
            {
                "nest": true
            }
        "#)
        .unwrap();

    let schema = json!({ "hello": { "world": "json" } }).into();
    let store = nest::Store::new(temp.path(), schema);

    assert_eq!(
        store.get(&["hello", "world", "nest"]).unwrap(),
        json!(true).into(),
    );

    assert_eq!(
        store.get(&["hello", "world"]).unwrap(),
        json!({ "nest": true }).into(),
    );

    assert_eq!(
        store.get(&["hello"]).unwrap(),
        json!({ "world": { "nest": true } }).into(),
    );

    assert_eq!(
        store.get(&[]).unwrap(),
        json!({ "hello": { "world": { "nest": true } } }).into(),
    );

    temp.close().unwrap();
}
