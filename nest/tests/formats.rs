extern crate assert_fs;

use std::convert::TryInto;

use assert_fs::prelude::*;
use serde_json::json;

use nest;

mod common;

#[test]
fn hjson() {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("hello/world.hjson");

    file.write_str(
        r#"{
  "nest": true
}"#,
    )
    .unwrap();

    let schema = json!({ "hello": { "world": "hjson" } }).try_into().unwrap();
    let store = nest::Store::new(temp.path(), schema);

    assert_eq!(
        store.get(&["hello", "world", "nest"]).unwrap(),
        json!(true).into(),
    );

    assert_eq!(
        store
            .set(&["hello", "world", "nest"], &json!(false).into())
            .unwrap(),
        (),
    );

    file.assert(
        r#"{
  nest: false
}"#,
    );

    temp.close().unwrap();
}

#[test]
fn yaml() {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("hello/world.yaml");

    file.write_str(
        r#"
nest: true
"#,
    )
    .unwrap();

    let schema = json!({ "hello": { "world": "yaml" } }).try_into().unwrap();
    let store = nest::Store::new(temp.path(), schema);

    assert_eq!(
        store.get(&["hello", "world", "nest"]).unwrap(),
        json!(true).into(),
    );

    assert_eq!(
        store
            .set(&["hello", "world", "nest"], &json!(false).into())
            .unwrap(),
        (),
    );

    file.assert(
        r#"---
nest: false
"#,
    );

    temp.close().unwrap();
}

#[test]
fn toml() {
    common::setup();

    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("hello/world.toml");

    file.write_str(
        r#"
nest = true
"#,
    )
    .unwrap();

    let schema = json!({ "hello": { "world": "toml" } }).try_into().unwrap();
    let store = nest::Store::new(temp.path(), schema);

    assert_eq!(
        store.get(&["hello", "world", "nest"]).unwrap(),
        json!(true).into(),
    );

    assert_eq!(
        store
            .set(&["hello", "world", "nest"], &json!(false).into())
            .unwrap(),
        (),
    );

    file.assert(
        r#"nest = false
"#,
    );

    temp.close().unwrap();
}
