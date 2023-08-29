use std::{fs::File, io::BufReader};

use crate::index::{Index, Term};

pub fn search(args: &str) -> Vec<Term> {
    let file = File::open("index.json");
    if let Err(_) = file {
        println!("Index file not found");
        return Vec::with_capacity(0);
    }
    let reader = BufReader::new(file.unwrap());
    let tree: Index = serde_json::from_reader(reader).expect("Failed to deserialize index file");
    tree.search(args)
}

pub fn load_index() -> Option<Index> {
    let file = File::open("index.json");
    if let Err(_) = file {
        println!("Index file not found");
        return None;
    }
    let reader = BufReader::new(file.unwrap());
    let index: Index = serde_json::from_reader(reader)
        .expect("Failed to deserialized");
    Some(index)
}
