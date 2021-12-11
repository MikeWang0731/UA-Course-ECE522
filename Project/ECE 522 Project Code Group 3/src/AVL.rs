use core::cmp::{max, Ordering};
use core::mem::swap;
use std::fmt::{Debug};

use DeleteValue::*;
use InnerResult::*;

pub type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;

#[derive(Clone, Debug)]
pub struct TreeNode<T: PartialOrd> {
    val: T,
    height: i32,
    left: AvlTreeNode<T>,
    right: AvlTreeNode<T>,
}

enum InnerResult {
    Left,
    // finished on left-side sub-tree
    Right,
    // finished on left-side sub-tree
    Unknown,
    // Unknown about balance
    Balanced,  // Tree Balanced
}

enum DeleteValue<T: PartialOrd> {
    Min,
    // Minimum node
    Max,
    // Maximum node
    Val(T),
    // "Input" value
    Del(AvlTreeNode<T>), // Deleted Node
}

impl<T: PartialOrd> PartialEq<Box<TreeNode<T>>> for DeleteValue<T> {
    fn eq(&self, other: &Box<TreeNode<T>>) -> bool {
        match self {
            Min => other.left.is_none(),
            Max => other.right.is_none(),
            Val(v) => v == &other.val,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd<Box<TreeNode<T>>> for DeleteValue<T> {
    fn partial_cmp(&self, other: &Box<TreeNode<T>>) -> Option<Ordering> {
        match self {
            Min => Some(Ordering::Less),
            Max => Some(Ordering::Greater),
            Val(v) => v.partial_cmp(&other.val),
            _ => None,
        }
    }
}

// private function trait
trait __AvlTree<T: PartialOrd> {
    fn right_rotate(&mut self);
    fn left_rotate(&mut self);
    fn rotate_lr(&mut self);
    fn rotate_rl(&mut self);
    fn update_height(&mut self);
    fn balance_factor(&self) -> i32;
    fn do_insert(&mut self, val: T) -> InnerResult;
    fn do_delete(&mut self, val: &mut DeleteValue<T>, val2: &T) -> InnerResult;
    fn recursive_print(&self, prefix_space: &String, is_right: bool, child_prefix: String);
    fn contains_node(&self, val: T) -> bool;
    fn inorder_to_list(&self, vec: &mut Vec<T>);
    fn preorder_to_list(&self, vec: &mut Vec<T>);
    fn postorder_to_list(&self, vec: &mut Vec<T>);
}

// public function trait - FOR USERS
pub trait AvlTree<T: PartialOrd> {
    fn new(val: T) -> Self;
    // generate a new node
    fn height(&self) -> i32;
    // get height of a node
    fn insert_node(&mut self, val: T);
    // insert node
    fn delete_node(&mut self, val: T) -> Self;
    // delete node
    fn validate_tree(&self) -> bool;
    // Is it a "balanced" AVL tree?
    fn is_tree_empty(&self) -> bool;
    // Tree empty?
    fn height_of_tree(&self) -> i32;
    // Height of this tree
    fn number_of_leaves(&self) -> i32;
    // number of leaves in this tree
    fn in_order_traverse(&mut self) -> Vec<T>;
    // in_order_traverse, return result as a vector
    fn pre_order_traverse(&mut self) -> Vec<T>;
    // pre_order_traverse, return result as a vector
    fn post_order_traverse(&mut self) -> Vec<T>;
    // post_order_traverse, return result as a vector
    fn print_tree_diagram(&mut self);
    // print the tree nicely
    fn exist_or_not(&self, val: T) -> bool;
    // check the existence of a specified node
    fn generate_empty_tree() -> Self;
    // generate a new empty tree
    fn update_node(&mut self, old: T, new: T);
    // update the node
    fn total_number_elements(&mut self) -> i32;
    // count total number of elements
}

// implementation for private
impl<T: PartialOrd + Copy + Debug> __AvlTree<T> for AvlTreeNode<T> {
    //         y                            x
    //        / \     Right Rotation       / \
    //       x  T4    ==============>     z   y
    //      / \                          /\ / \
    //     z  T3                        1 2 3  4
    //   T1 T2
    fn right_rotate(&mut self) {  // Case LL
        match self {
            Some(root) => {  // y is root
                // 1. Get the left subtree of root, i.e. the x branch (at this point the left subtree has been stripped)
                let left = &mut root.left.take();  // Get the value inside Option<>, leave a None
                match left {
                    // If the left subtree (x-branch) has something
                    Some(node) => {
                        // 2. T3 is connected to the left side of y (the left side of root and the right side of x are swapped)
                        // root.left=x.right & x.right=root.left
                        swap(&mut root.left, &mut node.right);
                        self.update_height();  // update height
                        // At this point self is y-(T3 & T4)
                        // 3. Connect y to the right side of the x branch (where root becomes x)
                        swap(self, &mut node.right);
                        // 4. Assign the reintegrated x branch (left) to self
                        // At this point self is the x branch after integration
                        swap(self, left);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    //         y                            x
    //        / \     Left  Rotation       / \
    //       T4  x    ==============>     y   z
    //          / \                      / \ / \
    //         T3  z                    4  3 2  1
    //           T2 T1
    fn left_rotate(&mut self) {  // Case RR
        match self {
            Some(root) => { // At this point root is y
                // 1. Get the right subtree of y, the x branch (at this point the subtree is stripped)
                let right = &mut root.right.take();
                match right {
                    // If x branch is not None
                    Some(node) => {
                        // 2. Swap the left side of x with the right side of y (i.e. y-(T4 & T3))
                        swap(&mut root.right, &mut node.left);
                        self.update_height();
                        // At this point self is y-(T4 & T3)
                        // 3. Connect the left side of x to self (i.e., the y branch), where root becomes x
                        swap(self, &mut node.left);
                        // 4. Assign the reintegrated x branch to the right variable
                        // At this point self is the x branch
                        swap(self, right);
                        self.update_height();
                    }
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    fn rotate_lr(&mut self) {
        match self {
            Some(root) => {
                root.left.left_rotate();
                self.right_rotate();
            }
            None => unreachable!(),
        }
    }

    fn rotate_rl(&mut self) {
        match self {
            Some(root) => {
                root.right.right_rotate();
                self.left_rotate();
            }
            None => unreachable!(),
        }
    }

    fn update_height(&mut self) {
        match self {
            None => {}
            // Find the highest height in the left subtree and the right subtree,
            // and add 1 to itself to be its own height
            Some(node) => node.height = max(node.left.height(), node.right.height()) + 1,
        }
    }

    fn balance_factor(&self) -> i32 {
        match self {
            None => 0,
            // Balance factor = left subtree height - right subtree height
            Some(node) => node.left.height() - node.right.height(),
        }
    }

    fn do_insert(&mut self, val: T) -> InnerResult {
        match self {
            // If there is no node at a given location, create a new one and put it here
            None => {
                *self = Self::new(val);
                Unknown
            }
            // Recursive insertion
            Some(root) => {
                // Duplicate data, do nothing
                if val == root.val {
                    Balanced
                } else if val < root.val {
                    // Target value < current node value, find position to the left subtree
                    match root.left.do_insert(val) {
                        Balanced => Balanced,
                        NotBalanced => {
                            // When the absolute value of "balance factor" is greater than 1, it is unbalanced,
                            // which is a positive number, representing the left side of the unbalance
                            if self.balance_factor() == 2 {
                                match NotBalanced {
                                    Left => self.right_rotate(), // Case: LeftLeft - ll
                                    Right => self.rotate_lr(), // Case LeftRight - lr
                                    _ => unreachable!(), // The current node must be balanced when `Unknown` is returned
                                }
                                Balanced  // The tree is balanced after the operation
                            } else if self.height() == {
                                // Verify that the height at this point (after rotate) is the same as the one recorded inside the node
                                self.update_height();
                                self.height()
                            } {
                                // Same as: else if self.height = self.height {Balanced}
                                Balanced
                            } else {
                                Left  // If the insertion is both balanced and the height is the same,
                                // record "left side finished inserting" directly
                            }
                        }
                    }
                    // Recursive insertion into the right subtree
                } else {
                    match root.right.do_insert(val) {
                        Balanced => Balanced,
                        NotBalanced => {
                            // When the absolute value of "equilibrium factor" is greater than 1, it is unbalanced,
                            // which is a negative number, representing the right side of the unbalance
                            if self.balance_factor() == -2 {
                                match NotBalanced {
                                    Left => self.rotate_rl(),  // case: RightLeft - rl
                                    Right => self.left_rotate(),  // case: RightRight - rr
                                    _ => unreachable!(),
                                }
                                Balanced
                            } else if self.height() == {
                                self.update_height();
                                self.height()
                            } {
                                Balanced
                            } else {
                                Right
                            }
                        }
                    }
                }
            }
        }
    }

    fn do_delete(&mut self, val: &mut DeleteValue<T>, val2: &T) -> InnerResult {
        // Core idea: Hibbard Deletion
        // When the node to be deleted is not empty, first find the subtree with the node to be deleted as the root,
        // and second find the node closest to its value and replace it with this node
        // e.g. I want to delete the node 59, the optimal solution is to find 58 or 60
        // (the left side to find the maximum, the right side to find the minimum)
        match self {
            // If the place has no value, then "do nothing"
            None => {
                *val = Del(None);
                println!("DELETE FAILED: No such node({:?}) to delete", val2);
                Balanced
            }
            // If have, then
            Some(root) => {
                // First get the height of the tree (or subtree) with this node as the heel, and save it as a backup
                let height = root.height;
                // case 1: If what you are looking for is the current
                if val == root {
                    if root.left.is_some() {
                        // Case 1-1: The left and right subtrees are not empty
                        if root.right.is_some() {
                            // Find the tallest subtree in the left or right sides to take the replacement node,
                            // reduce the damage to the balance
                            if root.left.height() > root.right.height() {
                                *val = Max;  // Give val a "Max tag"
                                root.left.do_delete(val, val2); // Delete the "largest node(Max)" in the left subtree and return this node
                                match val {
                                    // If there is a return value Del<Node<T>>, swap the "largest node(Max)" with the "node to be deleted",
                                    // so that the "largest node" enters the original position of the "node to be deleted".
                                    Del(Some(node)) => {
                                        // root.val -> the node we want to delete
                                        // node.val -> the node used to replace the "delete node"
                                        swap(&mut root.val, &mut node.val);
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                // else, find the minimum value in the right side
                                *val = Min;
                                root.right.do_delete(val, val2);  // delete and return the value
                                match val {
                                    // Same above
                                    Del(Some(x)) => {
                                        swap(&mut root.val, &mut x.val);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        } else {  // Case1-2: left.is_some() & right.is_none()
                            // Directly take the left-side subtree
                            let mut left = root.left.take();
                            // put it to the root's position
                            swap(self, &mut left);
                            *val = Del(left);  // return the deleted value
                        }
                    } else {  // left is none
                        // Directly take the right-side subtree
                        let mut right = root.right.take();
                        // put it to the root's position
                        swap(self, &mut right);
                        *val = Del(right); // return the deleted value
                    }
                    self.update_height();  // update the height
                } else if val < root {  // Case 2: val < root's value, go to the left-side
                    match root.left.do_delete(val, val2) {  // Recursively delete on the left subtree,
                        // when found after the node to be deleted will execute the code of Case 1 and return the result (balance or not)
                        Balanced => return Balanced,
                        Unknown => {  // If the imbalance will be self-rotating to maintain balance
                            if self.balance_factor() == -2 {  // The right side will be taller after the left side is deleted
                                let right = self.as_ref().unwrap().right.as_ref().unwrap();  // get right side
                                if right.left.height() > right.right.height() {  // if right-left is taller than right-right
                                    self.rotate_rl();  // RightLeft - rl case
                                } else {
                                    self.left_rotate();  // Otherwise, RightRight - rr case
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {  // Case 3: val > root's value, go to the right-side
                    match root.right.do_delete(val, val2) {
                        Balanced => return Balanced,
                        Unknown => {
                            if self.balance_factor() == 2 {
                                let left = self.as_ref().unwrap().left.as_ref().unwrap();
                                if left.left.height() >= left.right.height() {
                                    self.right_rotate();
                                } else {
                                    self.rotate_lr();
                                }
                            } else {
                                self.update_height();
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                // Here is the result of the recursion to the "bottom" level after performing the delete action
                // and returning it to the top level
                // root.[direction].do_delete(val) -> Balanced or Unknown?
                if self.height() == height {
                    Balanced
                } else {
                    Unknown
                }
            }
        }
    }

    fn recursive_print(&self, prefix_space: &String, is_left: bool, child_prefix: String) {
        if self.is_none() {
            let null_prefix = if is_left { "├ " } else { "└ " };
            println!("{}{}{} {}", prefix_space, null_prefix, child_prefix, "null");
            return;
        }

        let node = self.as_ref().unwrap();
        let prefix_current = if is_left { "├ " } else { "└ " };

        // Print the current
        println!(
            "{}{}{} {:?} : {}",
            prefix_space, prefix_current, child_prefix, self.as_ref().unwrap().val, node.height
        );

        // adjust the space
        let prefix_child = if is_left { "| " } else { "  " };
        let mut prefix_space = prefix_space.to_owned();
        prefix_space.push_str(&prefix_child);

        self.as_ref().unwrap().left.recursive_print(&prefix_space, true, "L ".to_string());
        self.as_ref().unwrap().right.recursive_print(&prefix_space, false, "R ".to_string());
    }

    fn contains_node(&self, val: T) -> bool {
        match self {
            None => {
                println!("Tree is empty, please add some node!");
                false
            }
            Some(_) => {
                if val.eq(&self.as_ref().unwrap().val) {
                    true
                } else if val.lt(&self.as_ref().unwrap().val) {
                    match &self.as_ref().unwrap().left {
                        None => false,
                        Some(_) => {
                            self.as_ref().unwrap().left.contains_node(val)
                        }
                    }
                } else {
                    match &self.as_ref().unwrap().right {
                        None => false,
                        Some(_) => {
                            self.as_ref().unwrap().right.contains_node(val)
                        }
                    }
                }
            }
        }
    }

    fn inorder_to_list(&self, vec: &mut Vec<T>) {
        if let Some(node) = self {
            self.as_ref().unwrap().left.inorder_to_list(vec);
            vec.push(node.val);
            self.as_ref().unwrap().right.inorder_to_list(vec);
        }
    }

    fn preorder_to_list(&self, vec: &mut Vec<T>) {
        if let Some(node) = self {
            vec.push(node.val);
            self.as_ref().unwrap().left.inorder_to_list(vec);
            self.as_ref().unwrap().right.inorder_to_list(vec);
        }
    }

    fn postorder_to_list(&self, vec: &mut Vec<T>) {
        if let Some(node) = self {
            self.as_ref().unwrap().left.inorder_to_list(vec);
            self.as_ref().unwrap().right.inorder_to_list(vec);
            vec.push(node.val);
        }
    }
}

// implementation for public
impl<T: PartialOrd + Copy + Debug> AvlTree<T> for AvlTreeNode<T> {
    // generate a new node
    fn new(val: T) -> Self {
        Some(Box::new(TreeNode {
            val,
            height: 1,
            left: None,
            right: None,
        }))
    }
    // get a node's height
    fn height(&self) -> i32 {
        match self {
            None => 0,
            Some(node) => node.height,
        }
    }
    // insert
    fn insert_node(&mut self, val: T) {
        self.do_insert(val);
    }
    // delete
    fn delete_node(&mut self, val: T) -> Self {
        let val2 = val.clone();
        let mut val = Val(val);
        self.do_delete(&mut val, &val2);
        match val {
            Del(node) => {
                if node.is_some() {
                    println!("Node({:?}) delete successfully.", val2);
                }
                node
            }
            _ => unreachable!()
        }
    }
    // Is it a avl tree?
    fn validate_tree(&self) -> bool {
        if let Some(root) = self {
            if root.height != max(root.left.height(), root.right.height()) + 1 {
                return false;
            }
            if self.balance_factor().abs() > 1 {
                return false;
            }
            if let Some(x) = &root.left {
                if !(x.val < root.val && self.as_ref().unwrap().left.validate_tree()) {
                    return false;
                }
            }
            if let Some(x) = &root.right {
                if !(x.val > root.val && self.as_ref().unwrap().right.validate_tree()) {
                    return false;
                }
            }
        }
        true
    }

    fn is_tree_empty(&self) -> bool {
        match self {
            None => true,
            Some(_) => false
        }
    }

    fn height_of_tree(&self) -> i32 {
        if self.is_none() {
            return 0;
        }
        let height_overall = self.as_ref().unwrap().height;
        height_overall
    }

    fn number_of_leaves(&self) -> i32 {
        let mut count = 0;
        if self.is_none() {
            count = 0;
        } else if self.as_ref().unwrap().left.is_none() && self.as_ref().unwrap().right.is_none() {
            count = 1;
        } else {
            count = &self.as_ref().unwrap().left.number_of_leaves() + &self.as_ref().unwrap().right.number_of_leaves();
        };
        count
    }

    fn in_order_traverse(&mut self) -> Vec<T> {
        let mut inorder_list = Vec::new();
        self.inorder_to_list(&mut inorder_list);
        inorder_list
    }

    fn pre_order_traverse(&mut self) -> Vec<T> {
        let mut preorder_list = Vec::new();
        self.preorder_to_list(&mut preorder_list);
        preorder_list
    }

    fn post_order_traverse(&mut self) -> Vec<T> {
        let mut postorder_list = Vec::new();
        self.postorder_to_list(&mut postorder_list);
        postorder_list
    }

    fn print_tree_diagram(&mut self) {
        match self {
            None => println!("Tree is Empty! Add some nodes before print."),
            Some(_) => {
                println!("\n================== TREE PRINT <Node:Height> ==================");
                self.recursive_print(&"".to_string(), true, "Root".to_string());
                println!("======================== FINISH PRINT ========================");
            }
        }
    }

    fn exist_or_not(&self, val: T) -> bool {
        match self {
            None => false,
            Some(_) => {
                self.contains_node(val)
            }
        }
    }

    fn generate_empty_tree() -> Self {
        Self::None
    }

    fn update_node(&mut self, old: T, new: T) {
        match self {
            None => println!("Tree is Empty! Add some nodes before update."),
            Some(_) => {
                if self.exist_or_not(old) == false {
                    println!("UPDATE FAILED: Node({:?}) doesn't exist!", old);
                } else if old == new {
                    println!("UPDATE FAILED: New value and old value can not be same!");
                } else {
                    if self.exist_or_not(old) && self.exist_or_not(new) {
                        println!("UPDATE FAILED: Both Node({:?}) and Node({:?}) exist!", old, new);
                    } else {
                        self.do_delete(&mut Val(old), &old);
                        self.insert_node(new);
                        println!("Node({:?}) has been replaced by Node({:?})", old, new);
                    }
                }
            }
        }
    }

    fn total_number_elements(&mut self) -> i32 {
        let res_vec = self.in_order_traverse();
        res_vec.len() as i32
    }
}
