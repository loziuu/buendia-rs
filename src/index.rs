use serde::{Deserialize, Serialize};

use super::splitter;

const ROOT_VALUE: &str = "\0";

// Trie with prefixes
// Single-threaded so far.
// TODO: Does it need to be sorted eventually?
#[derive(Serialize, Deserialize)]
pub struct Index {
    root: Node,
}

#[derive(Serialize, Deserialize)]
struct Node {
    term: String,
    children: Vec<Node>,
    positions: Vec<usize>,
    is_word: bool,
}

impl Node {
    pub fn new_word(term: &str, position: usize) -> Node {
        Node {
            term: term.to_string(),
            children: vec![],
            positions: vec![position],
            is_word: true,
        }
    }

    pub fn new(term: &str) -> Node {
        Node {
            term: term.to_string(),
            children: vec![],
            positions: vec![],
            is_word: false,
        }
    }

    pub fn flatten(&self) -> Vec<Term> {
        let mut terms = Vec::new();
        let mut stack = Vec::new();
        stack.push(self);
        while let Some(node) = stack.pop() {
            if node.is_word {
                terms.push(Term {
                    term: node.term.clone(),
                    positions: node.positions.clone(),
                });
            }
            stack.extend(node.children.iter());
        }
        terms
    }
}

pub struct Term {
    pub term: String,
    pub positions: Vec<usize>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            root: Node {
                term: ROOT_VALUE.to_owned(),
                children: vec![],
                positions: vec![],
                is_word: false,
            },
        }
    }

    pub fn insert(&mut self, term: String, position: usize) {
        println!("Inserting: {}", term);
        insert_into_tree(&mut self.root, &term, position);
    }

    pub fn find_term(&self, term: &str) -> Option<Term> {
        find(&self.root, term).map(|n| Term {
            term: n.term.clone(),
            positions: n.positions.clone(),
        })
    }

    pub fn search(&self, arg: &str) -> Vec<Term> {
        if let Some(node) = traverse_tree(&self.root, arg) {
            println!("Found term: {}", node.term);
            node.flatten()
        } else {
            Vec::with_capacity(0)
        }
    }

    pub fn flat_terms(&self) -> Vec<Term> {
        self.root.flatten()
    }
}

fn insert_into_tree(node: &mut Node, term: &str, position: usize) {
    // This is linear now. Can it be binary search once sorted?
    let children = &mut node.children;
    for i in 0..children.len() {
        let n = &mut children[i];
        if n.term == term {
            n.is_word = true;
            return n.positions.push(position);
        }
        let prefix = splitter::prefix_split_only_position(&n.term, term);
        if prefix == n.term.len() {
            return insert_into_tree(n, &term[prefix..], position);
        }
        if prefix > 0 {
            if prefix == term.len() {
                let mut prefix_node = Node::new_word(&term, position);
                let mut removed = children.remove(i);
                removed.term = removed.term[prefix..].to_string();
                prefix_node.children.push(removed);
                children.insert(i, prefix_node);
                return;
            } else {
                println!("Splitting node: {} at: {}", n.term, prefix);
                let mut prefix_node = Node::new(&n.term[..prefix]);
                let mut removed = children.remove(i);
                removed.term = removed.term[prefix..].to_string();

                insert_into_tree(&mut prefix_node, &term[prefix..], position);

                prefix_node.children.push(removed);
                children.insert(i, prefix_node);
                return;
            }
        }
    }
    return children.push(Node::new_word(term, position));
}

fn traverse_tree<'a>(node: &'a Node, term: &str) -> Option<&'a Node> {
    println!("Traversing node: {} for term: {}", node.term, term);
    if node.term == term {
        Some(node)
    } else {
        let last = splitter::prefix_split_only_position(term, &node.term);
        if last == 0 && node.term != ROOT_VALUE {
            return None;
        }
        if last == term.len() {
            println!("Found term: {}", node.term);
            return Some(node);
        }
        for i in 0..node.children.len() {
            let slice = &term[last..];

            if let Some(n) = traverse_tree(&node.children[i], &slice) {
                return Some(n);
            }
        }
        None
    }
}

fn find<'a>(node: &'a Node, term: &str) -> Option<&'a Node> {
    if node.term == term {
        Some(node)
    } else {
        let last = splitter::prefix_split_only_position(term, &node.term);
        if last == 0 && node.term != ROOT_VALUE {
            return None;
        }
        for i in 0..node.children.len() {
            let slice = &term[last..];
            if let Some(n) = find(&node.children[i], &slice) {
                return Some(n);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn insert_into_index() {
        let mut tree = Index::new();
        let val = String::from("example");

        tree.insert(val, 1);

        assert_eq!(tree.root.children.len(), 1);
        tree.root.children.iter().for_each(|node| {
            assert_eq!(node.term, "example");
            assert_eq!(node.positions, vec![1]);
        });
    }

    #[test]
    fn insert_into_tree_existing_term_second_position() {
        let mut tree = Index::new();
        let val = String::from("example");

        tree.insert(val.clone(), 1);
        tree.insert(val, 2);

        assert_eq!(tree.root.children.len(), 1);
        tree.root.children.iter().for_each(|node| {
            assert_eq!(node.term, "example");
            assert_eq!(node.positions, vec![1, 2]);
        });
    }

    #[test]
    fn insert_into_tree_and_traverse() {
        let mut tree = Index::new();
        tree.insert(String::from("example"), 10);

        tree.find_term("example").map(|n| {
            assert_eq!(n.term, "example");
            assert_eq!(n.positions, vec![10]);
        });
    }

    #[test]
    fn insert_two_different_terms() {
        let mut tree = Index::new();
        tree.insert(String::from("first"), 10);
        tree.insert(String::from("second"), 10);

        assert_eq!(tree.root.children.len(), 2);
    }

    #[test]
    fn insert_to_with_same_prefix() {
        let mut tree = Index::new();
        tree.insert(String::from("example"), 10);
        tree.insert(String::from("exactly"), 10);

        let result = tree.search("exa");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn insert_to_with_same_prefixes() {
        let mut tree = Index::new();
        tree.insert(String::from("example"), 10);
        tree.insert(String::from("exactly"), 10);
        tree.insert(String::from("experience"), 10);
        tree.insert(String::from("electric"), 10);

        let result = tree.search("e");

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_aragorn() {
        let mut tree = Index::new();
        tree.insert(String::from("aragorn"), 1);
        tree.insert(String::from("aragorn"), 2);
        tree.insert(String::from("arathorn"), 3);
        tree.insert(String::from("aragorn"), 4);

        let result = tree.find_term("aragorn");
        assert_eq!(result.unwrap().positions.len(), 3);
    }

    #[test]
    fn insert_to_with_same_prefixes_and_traverse() {
        let mut tree = Index::new();
        tree.insert(String::from("example"), 10);
        tree.insert(String::from("exactly"), 10);
        tree.insert(String::from("experience"), 10);
        tree.insert(String::from("electric"), 10);
        tree.insert(String::from("efficiency"), 10);

        assert_eq!(tree.search("e").len(), 5);
        assert_eq!(tree.search("ex").len(), 3);
        assert_eq!(tree.search("exa").len(), 2);
        assert_eq!(tree.search("exac").len(), 1);
        assert_eq!(tree.search("exactly").len(), 1);
        assert_eq!(tree.search("efficiency").len(), 1);
    }
}
