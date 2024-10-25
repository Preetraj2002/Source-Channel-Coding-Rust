use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Node {
    character: Option<char>,
    frequency: u32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_huffmann_tree(freq_map: HashMap<char, u32>) -> Option<Rc<RefCell<Node>>> {
    let mut heap = BinaryHeap::new();
    // create a new node for each character and add it to leaf
    for (ch, freq) in freq_map {
        let node = Rc::new(RefCell::new(Node {
            character: Some(ch),
            frequency: freq,
            left: None,
            right: None,
        }));
        heap.push(node);
    }

    // merge two lowest frequency node to build tree
    while heap.len() > 1 {
        let left = heap.pop()?;
        let right = heap.pop()?;

        let new_node = Rc::new(RefCell::new(Node {
            character: None,
            frequency: left.clone().borrow().frequency + right.clone().borrow().frequency,
            left: Some(left),
            right: Some(right),
        }));
        heap.push(new_node);
    }
    // return the root of the tree
    heap.pop()
}

pub fn generate_huffmann_codes(
    node: &Option<Rc<RefCell<Node>>>,
    prefix: String,
    codes: &mut HashMap<char, String>,
) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        if let Some(ch) = node.character {
            codes.insert(ch, prefix);
        } else {
            generate_huffmann_codes(&node.left, format!("{}0", prefix), codes);
            generate_huffmann_codes(&node.right, format!("{}1", prefix), codes);
        }
    }
}
