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

#[test]
fn set_simple () {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp
        .child("hello/world.json")
        .write_str(r#"
            {
                "nest": 0
            }
        "#)
        .unwrap();

    let schema = json!({ "hello": { "world": "json" } }).into();
    let store = nest::Store::new(temp.path(), schema);

    assert_eq!(
        store.set(&["hello", "world", "nest"], &json!(1).into()).unwrap(),
        (),
    );
    assert_eq!(
        store.get(&["hello", "world", "nest"]).unwrap(),
        json!(1).into(),
    );

    assert_eq!(
        store.set(&["hello", "world"], &json!({ "nest": 2 }).into()).unwrap(),
        (),
    );
    assert_eq!(
        store.get(&["hello", "world"]).unwrap(),
        json!({ "nest": 2 }).into(),
    );

    assert_eq!(
        store.set(&["hello"], &json!({ "world": { "nest": 3 } }).into()).unwrap(),
        (),
    );
    assert_eq!(
        store.get(&["hello"]).unwrap(),
        json!({ "world": { "nest": 3 } }).into(),
    );

    assert_eq!(
        store.set(&[], &json!({ "hello": { "world": { "nest": 4 } } }).into()).unwrap(),
        ()
    );
    assert_eq!(
        store.get(&[]).unwrap(),
        json!({ "hello": { "world": { "nest": 4 } } }).into(),
    );

    temp.close().unwrap();
}
