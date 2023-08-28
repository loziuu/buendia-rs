use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::index::{self, Index};

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
        let start = std::time::Instant::now();
        let file = File::open(path).map_err(|_| IndexerError::FileNotFound)?;
        let reader = BufReader::new(file);
        let mut position: usize = 0;
        let mut index = Index::new(); 

        for line in reader.lines() {
            let line = line.unwrap();
            for token in line.split_whitespace() {
                println!("{}: {}", position, token);
                position = position+1;
                index.insert(token.to_string(), position);
            }
        }

        let index_file =
            File::create("index.txt").map_err(|_| IndexerError::CouldntCreateIndexFile)?;
        let mut writer = BufWriter::new(index_file);

        //   Serialize tree to file
     //   for (token, positions) in map {
      //      writer
       //         .write_fmt(format_args!("{}: {}\n", token, positions.join(",")))
        //        .map_err(|_| IndexerError::FailedToWriteToIndexFile)?;
         //   writer
          //      .flush()
           //     .map_err(|_| IndexerError::FailedToFlushIndexFile)?;
       // }

        print!("Indexing took {}ms", start.elapsed().as_millis());
        Ok(())
    }
}
