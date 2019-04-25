use common_macros::b_tree_map;
use nest::{Database, Error, Store, Value};

fn main() {
    let store = Store::Tree(b_tree_map! {
        "example-data".into() => Store::Tree(b_tree_map! {
            "foo".into() => Store::Json
        })
    });
    let database = Database::new(
        store
    );
    let foobar = database.get(vec!["example-data".into(), "foo".into(), "bar".into()]).unwrap();
    println!("foobar {:?}", foobar);

    database.set(vec!["example-data".into(), "foo".into(), "bar".into()], &Value::String("hello".into())).unwrap();

    let ab = database.get(vec!["a".into(), "b".into()]);
    println!("ab {:?}", ab);
}

