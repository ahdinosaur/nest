use nest::{Store, Value};

use serde_json::json;

fn main() {
    // what is the root path to your data store?
    let root = "./";
    // describe how your data store will map to the filesystem
    let schema = json!({
        // refers to a directory: ./example-data/
        "example-data": {
            // refers to a file: ./example-data/foo.json
            "foo": "json"
        }
    }).into();

    let store = Store::new(root, schema);

    // get `bar` key from `./example-data/foo.json` file
    let bar = store.get(&["example-data", "foo", "bar"]).unwrap();
    println!("bar {:?}", bar);

    // set `bar` key in `./example-data/foo.json` file
    let next_bar = &Value::String("baz".into());
    store.set(&["example-data", "foo", "bar"], next_bar).unwrap();

    // get a sub-store for data within `./example-data/foo.json
    let foo = store.sub(&["example-data", "foo"]).unwrap();
    let bar = foo.get(&["bar"]).unwrap();
    println!("bar {:?}", bar);

    // try to get a value that doesn't map to the schema
    let err = store.get(&["invalid", "path"]);
    println!("err {:?}", err);
}
