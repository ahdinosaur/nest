use nest::{Database, Value};

use serde_json::json;

fn main() {
    let store = json!({
        "example-data": {
            "foo": "json"
        }
    }).into();
    let database = Database::new(
        store
    );
    let foobar = database.get(vec!["example-data".into(), "foo".into(), "bar".into()]).unwrap();
    println!("foobar {:?}", foobar);

    database.set(vec!["example-data".into(), "foo".into(), "bar".into()], &Value::String("hello".into())).unwrap();

    let ab = database.get(vec!["a".into(), "b".into()]);
    println!("ab {:?}", ab);
}

