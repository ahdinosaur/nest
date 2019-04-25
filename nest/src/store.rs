use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Store {
    Tree(BTreeMap<String, Store>),
    Json
}
