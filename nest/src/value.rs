use indexmap::IndexMap;

/// Represents any valid Nest value.
///
/// Similar to [`serde_json::Value`].
///
/// `Value` also implements `From<serde_json::Value>`, so you can use the [`serde_json::json`] macro
/// to conveniently create Nest values.
///
/// [`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
/// [`serde_json::json`]: https://docs.serde.rs/serde_json/macro.json.html

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Value {
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }
}
