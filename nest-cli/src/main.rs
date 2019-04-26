use nest::{Store, Value};

use serde_json::json;

fn main() {
    let schema = json!({
        "example-data": {
            "foo": "json"
        }
    }).into();
    let root = "./";
    let store = Store::new(schema, root);

    // TODO change to use &str's
    let foobar = store.get(vec!["example-data", "foo", "bar"]).unwrap();
    println!("foobar {:?}", foobar);

    store.set(vec!["example-data", "foo", "bar"], &Value::String("hello".into())).unwrap();

    let ab = store.get(vec!["a", "b"]);
    println!("ab {:?}", ab);
}

// dyndns-host
// config = db.sub(['dyndns', 'host'])
// config.get(['guests'])
// config.set(['guests'], 'butt')

// mix = db.get(['humans', 'mix'])
// mix.get()
// mix.set({ name: 'Mix' })
//
// db.set(['humans', 'mix'], { name: 'Mix' })
//
// immutable-rs
