use common_macros::hash_map;
use std::collections::HashMap;

fn main() {
    let structure = Structure::Tree(hash_map! {
        "foo".into() => Structure::Tree(hash_map! {
            "bar".into() => Structure::Json
        })
    });
    let database = Database::new(
        structure
    );
    database.get(vec!["foo".into(), "bar".into()]);
}

struct Database {
    structure: Structure
}

impl Database {
    pub fn new (structure: Structure) -> Self {
        Database {
            structure
        }
    }

    pub fn get (&self, path: Vec<String>) {
        let store = get_in_structure(&path, &self.structure);
            
        println!("{:?} {:?}", path, store);
    }
    
}

fn get_in_structure<'a> (path: &[String], structure: &'a Structure) -> Option<&'a Structure> {
    match structure {
        Structure::Tree(map) => {
            if path.len() == 0 { return None }
            let key = path.get(0).unwrap();
            let next_path = path.get(1..path.len()).unwrap();
            let next_structure = map.get(key).unwrap();
            get_in_structure(next_path, next_structure)
        },
        leaf => Some(leaf)
    }
}

#[derive(Debug)]
enum Structure {
    Tree(HashMap<String, Structure>),
    Json
}
