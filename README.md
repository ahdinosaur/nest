<h1 align="center">Nest 🐦</h1>

<div align="center">
  <strong>
    Use your filesystem as a nested data store!
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/nest">
    <img src="https://img.shields.io/crates/v/nest.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Build Status -->
  <a href="https://travis-ci.org/ahdinosaur/nest">
    <img src="https://img.shields.io/travis/ahdinosaur/nest.svg?style=flat-square"
      alt="Build Status" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/nest">
    <img src="https://img.shields.io/crates/d/nest.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/nest">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

## Example

```rust
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
```


## Contributing

### Conduct

The Nest project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/1/4/code-of-conduct). This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the [Apache-2.0 license](LICENSE-APACHE) and [Developer Certificate of Origin](CERTIFICATE), shall be dual licensed as above, without any additional terms or conditions.
