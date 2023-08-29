use buendia_rs::{index::Index, searcher};

fn main() {
    let mut index = searcher::load_index().expect("failed to load index");

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.starts_with("insert:") {
            let input = input.replace("insert:", "");
            let mut split = input.split_whitespace();
            let term = split.next().unwrap();
            let position = 1;
            index.insert(term.to_string(), position);
            println!("Inserted {} at {}", term, position);
            println!("============");
            continue;
        }

        if input.starts_with("eq:") {
            let term = input.replace("eq:", "");
            let result = index.find_term(&term);
            match result {
                Some(node) => {
                    println!("Found at {:?}", node.positions);
                    println!("{} was found {} times.", node.term, node.positions.len());
                }
                None => {
                    println!("Term not found");
                }
            }
            continue;
        }

        if input == "all" {
            println!("All terms:");
            index.flat_terms().iter().for_each(|term| {
                println!("{}: {:?}", term.term, term.positions);
            });
            println!("============");
            continue;
        }

        let mut terms = index.search(input);
        terms.sort_by(|a, b| b.positions.len().cmp(&a.positions.len()));
        terms.iter().for_each(|term| {
            println!("{} was found {} times.", term.term, term.positions.len());
            println!("{}: {:?}", term.term, term.positions);
        });
    }
}
