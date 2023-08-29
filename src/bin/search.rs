use std::env;

use buendia_rs::searcher;

fn main() {
    let phrase = env::args().nth(1)
        .expect("Please provide a search phrase.");
    let terms = searcher::search(&phrase);
    println!("Found {} results for {}:", terms.len(), phrase);
    terms.iter().for_each(|term| {
        println!("{}: {:?}", term.term, term.positions);
    });
}

