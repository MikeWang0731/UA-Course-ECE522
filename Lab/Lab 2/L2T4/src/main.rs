use crate::Tree::Node;
use std::cmp::Ordering;

#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    // init a node
    fn new() -> Tree<T> {
        Tree::Empty
    }

    fn insert(&mut self, new_val: T) {
        // pattern matching
        match self {
            // Find the destination to put the node
            // if there is a node, find its child(do recursively)
            &mut Tree::Node { ref data, ref mut left_child, ref mut right_child } => {
                match new_val.cmp(data) {
                    Ordering::Less => left_child.insert(new_val),
                    Ordering::Greater => right_child.insert(new_val),
                    _ => return
                }
            }
            // if there is no node, put the new node here
            &mut Tree::Empty => {
                *self = Tree::Node { data: new_val, left_child: Box::new(Tree::Empty), right_child: Box::new(Tree::Empty) }
            }
        }
    }
}

fn main() {
    let mut node_1 = Tree::Node {
        data: 5,
        left_child: Box::new(Tree::Empty),
        right_child: Box::new(Tree::Empty),
    };
    node_1.insert(4);
    node_1.insert(6);
    println!("{:?}", node_1);
}