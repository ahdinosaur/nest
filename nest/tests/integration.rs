extern crate assert_fs;

use assert_fs::prelude::*;
use serde_json::json;

use nest;

mod common;

#[test]
fn get_simple () {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    temp
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
    temp
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

#[test]
fn set_from_empty () {
    common::setup();

    let schema: nest::Schema = json!({ "hello": { "world": "json" } }).into();
    let expected =
r#"{
  "nest": true
}
"#;

    let temp_0 = assert_fs::TempDir::new().unwrap();
    let store = nest::Store::new(temp_0.path(), schema.clone());
    assert_eq!(
        store.set(&["hello", "world", "nest"], &json!(true).into()).unwrap(),
        (),
    );
    temp_0
        .child("hello/world.json")
        .assert(expected);
    temp_0.close().unwrap();

    let temp_1 = assert_fs::TempDir::new().unwrap();
    let store = nest::Store::new(temp_1.path(), schema.clone());
    assert_eq!(
        store.set(&["hello", "world"], &json!({ "nest": true }).into()).unwrap(),
        (),
    );
    temp_1
        .child("hello/world.json")
        .assert(expected);
    temp_1.close().unwrap();

    let temp_2 = assert_fs::TempDir::new().unwrap();
    let store = nest::Store::new(temp_2.path(), schema.clone());
    assert_eq!(
        store.set(&["hello"], &json!({ "world": { "nest": true } }).into()).unwrap(),
        (),
    );
    temp_2
        .child("hello/world.json")
        .assert(expected);
    temp_2.close().unwrap();

    let temp_3 = assert_fs::TempDir::new().unwrap();
    let store = nest::Store::new(temp_3.path(), schema.clone());
    assert_eq!(
        store.set(&[], &json!({ "hello": { "world": { "nest": true } } }).into()).unwrap(),
        (),
    );
    temp_3
        .child("hello/world.json")
        .assert(expected);
    temp_3.close().unwrap();
}
