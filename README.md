<h1 align="center">Nest 🐦</h1>

<div align="center">
  <strong>
    Use your filesystem as a nested data store!
  </strong>
</div>

## Modules

- `nest`: [![Crates.io version](https://img.shields.io/crates/v/nest.svg?style=flat-square) ](https://crates.io/crates/nest)  [ ![Build Status](https://img.shields.io/travis/ahdinosaur/nest.svg?style=flat-square) ](https://travis-ci.org/ahdinosaur/nest)  [ ![Download](https://img.shields.io/crates/d/nest.svg?style=flat-square) ](https://crates.io/crates/nest)  [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/nest)
- `nest-cli`: [![Crates.io version](https://img.shields.io/crates/v/nest-cli.svg?style=flat-square) ](https://crates.io/crates/nest-cli)  [ ![Build Status](https://img.shields.io/travis/ahdinosaur/nest.svg?style=flat-square) ](https://travis-ci.org/ahdinosaur/nest)  [ ![Download](https://img.shields.io/crates/d/nest-cli.svg?style=flat-square) ](https://crates.io/crates/nest-cli)  [![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/nest-cli)

## `nest`

### Example

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

## `nest-cli`

### Install

```shell
cargo install nest-cli
```

### Example

Given [`example-data`](./example-data), containing a [`.nest.json`](./example-data/nest.json):

```shell
cd example-data

nest get 'hello/world'
# {
#   "nest": "🐣"
# }

nest set 'hello/world/nest' '"🐥"'

nest get 'hello'
# {
#   "world": {
#     "nest": "🐣"
#   }
# }
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
