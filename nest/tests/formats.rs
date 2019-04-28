extern crate assert_fs;

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

    let schema = json!({ "hello": { "world": "hjson" } }).into();
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
