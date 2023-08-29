use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    time::Instant,
};

use crate::{index::Index, tokenizer};

pub type IndexerResult = Result<(), IndexerError>;

#[derive(Debug)]
pub enum IndexerError {
    FileNotFound,
    CouldntCreateIndexFile,
    FailedToWriteToIndexFile,
    FailedToFlushIndexFile,
}

pub struct Indexer {}

impl Indexer {
    pub fn new() -> Indexer {
        Indexer {}
    }

    pub fn index(&self, path: &str) -> IndexerResult {
        println!("Indexing {}", path);
        let indexing_start = std::time::Instant::now();
        let file = File::open(path).map_err(|_| IndexerError::FileNotFound)?;
        let reader = BufReader::new(file);
        let mut position: usize = 0;
        let mut index = Index::new();

        for line in reader.lines() {
            let line = line
                .unwrap();
            for token in tokenizer::tokenize(&line) {
                position = position + 1;
                println!("Inserting {} at {}", token, position);
                index.insert(token.to_string(), position);
            }
        }

        println!("Indexing took {}ms", indexing_start.elapsed().as_millis());
        let disk_dump_start = Instant::now();
        println!("Dumping index to disk",);
        dump_to_disk(index)?;
        println!(
            "Dumping index to disk took {}ms",
            disk_dump_start.elapsed().as_millis()
        );
        Ok(())
    }
}

fn dump_to_disk(tree: Index) -> IndexerResult {
    let index_file =
        File::create("index.json").map_err(|_| IndexerError::CouldntCreateIndexFile)?;
    let mut writer = BufWriter::new(index_file);

    serde_json::to_writer(&mut writer, &tree)
        .map_err(|_| IndexerError::FailedToWriteToIndexFile)?;
    writer
        .flush()
        .map_err(|_| IndexerError::FailedToFlushIndexFile)?;
    Ok(())
}
