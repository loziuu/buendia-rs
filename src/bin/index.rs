use std::env;

use buendia_rs::indexer::Indexer;

fn main() {
    let file_name = env::args().nth(1)
        .expect("Please provide a file name");
    let indexer = Indexer::new();
    indexer.index(&file_name).unwrap();
}
