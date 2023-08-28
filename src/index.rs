use super::splitter;

// Trie with prefixes 
// Single-threaded so far.
// TODO: Does it need to be sorted eventually?
pub struct Index {
    root: Node,
}

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
                terms.push(Term{ term: node.term.clone(), positions: node.positions.clone()} );
            }
            stack.extend(node.children.iter());
        }
        terms
    }
}

struct Term {
    term: String,
    positions: Vec<usize>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            root: Node {
                term: "\0".to_string(),
                children: vec![],
                positions: vec![],
                is_word: false,
            },
        }
    }

    pub fn insert(&mut self, term: String, position: usize) {
        insert_into_tree(&mut self.root, &term, position);
    }

    pub fn find_term(&self, term: &str) -> Option<Term> {
        traverse_tree(&self.root, term).map(|n| Term {
            term: n.term.clone(),
            positions: n.positions.clone(),
        })
    }

    fn search(&self, arg: &str) -> Vec<Term> {
        if let Some(node) = traverse_tree(&self.root, arg) {
            println!("Found term: {}", node.term);
            node.flatten()
        } else {
            Vec::with_capacity(0)
        }
    }
}

fn insert_into_tree(node: &mut Node, term: &str, position: usize) {
    // This is linear now. Can it be binary search once sorted?
    let children = &mut node.children;
    for i in 0..children.len() {
        let n = &mut children[i];
        let prefix = splitter::prefix_split_only_position(&n.term, term);
        if prefix == n.term.len() {
            return n.positions.push(position);
        }
        if prefix > 0 {
            let mut prefix_node = Node::new(&n.term[..prefix]);
            let mut removed = children.remove(i);
            removed.term = removed.term[prefix..].to_string();

            insert_into_tree(&mut removed, &term[prefix..], position);

            prefix_node.children.push(removed);
            children.insert(i, prefix_node);
            return;
        }
    }
    return children.push(Node::new_word(term, position));
}

fn traverse_tree<'a>(node: &'a Node, term: &str) -> Option<&'a Node> {
    if node.term == term {
        Some(node)
    } else {
        if node.children.is_empty() {
            return None;
        }
        let last = splitter::prefix_split_only_position(term, &node.term);
        for i in 0..node.children.len() {
            let slice = &term[last..];
            if let Some(n) = traverse_tree(&node.children[i], &slice) {
                return Some(n);
            }
        }
        None
    }
}

fn closest_term<'a>(node: &'a Node, arg: &str) -> Option<&'a Node> {
    let prefix = splitter::prefix_split_only_position(arg, &node.term);
    if prefix == node.term.len() || prefix == arg.len() {
        Some(node)
    } else if !node.children.is_empty() {
        for i in 0..node.children.len() {
            let slice = &arg[prefix..];
            if let Some(n) = traverse_tree(&node.children[i], &slice) {
                return Some(n);
            }
        }
        None
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::index::Node;

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
}
