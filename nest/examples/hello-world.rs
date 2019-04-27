use nest::{Store, Error, Value};

use serde_json::json;

fn main () -> Result<(), Error> {
    // what is the root path to your data store?
    let root = "./example-data";
    // describe how your data store will map to the filesystem
    let schema = json!({
        // refers to a directory: ./example-data/hello/
        "hello": {
            // refers to a file: ./example-data/hello/world.json
            "world": "json"
        }
    }).into();

    let store = Store::new(root, schema);

    // get `nest` key from `./example-data/hello/world.json` file
    let value = store.get(&["hello", "world", "nest"])?;
    println!("value: {:?} == 🐣", value);

    // set `nest` key in `./example-data/hello/world.json` file
    let next_value = &Value::String("🐥".into());
    store.set(&["hello", "world", "nest"], next_value)?;

    // get a sub-store for data within `./example-data/hello/world.json
    let sub = store.sub(&["hello", "world"])?;
    let value = sub.get(&["nest"])?;
    println!("value: {:?} == 🐥", value);

    // try to get a value that doesn't map to the schema
    let err = store.get(&["invalid", "path"]);
    println!("err {:?}", err);

    Ok(())
}
