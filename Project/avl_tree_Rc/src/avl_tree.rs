use std::borrow::{Borrow};
use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Debug;
use std::mem::swap;
use std::rc::Rc;

pub type Node<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug)]
pub struct TreeNode<T: PartialOrd + Copy + Debug> {
    value: T,
    height: i32,
    left: Node<T>,
    right: Node<T>,
}

pub trait Avl<T: PartialOrd + Copy + Debug> {
    fn generate_new_node(val: T) -> Self;
    fn generate_new_tree() -> Self;
    // 新建树
    fn insert_node(&mut self, val: T);
    // 添加
    fn delete_node(&mut self, val: T);
    // 删除
    fn find_minimum_node(&mut self) -> Node<T>;
    fn update_node_height(&mut self);
    fn get_node_height(&self) -> i32;
    fn get_tree_height(&self) -> i32;
    // 找树的高度
    fn get_balance_factor(&self) -> i32;
    fn right_rotate(&mut self);
    fn left_rotate(&mut self);
    fn left_right_rotate(&mut self);
    fn right_left_rotate(&mut self);
    fn inorder(&self, vec: &mut Vec<T>);
    fn get_inorder(&self) -> Vec<T>;
    // 中序遍历
    fn count_leaves(&self) -> i32;
    // 找叶子数量
    fn is_empty(&self) -> bool;
    // 树是空的吗
    fn print(&self, prefix: &String);
    fn pretty_print(&self);
    // 打印树
    fn is_exist(&self, val: T) -> bool;
    // 节点是否存在
}

impl<T: PartialOrd + Copy + Debug> Avl<T> for Node<T> {
    fn generate_new_node(val: T) -> Self {
        Some(Rc::new(
            RefCell::new(
                TreeNode {
                    value: val,
                    height: 1,
                    left: None,
                    right: None,
                }
            )
        ))
    }

    fn generate_new_tree() -> Self {
        Self::None
    }

    fn insert_node(&mut self, val: T) {
        if self.is_none() {
            *self = Self::generate_new_node(val);
        } else {
            if val < self.as_ref().unwrap().borrow_mut().value {
                self.as_ref().unwrap().borrow_mut().left.insert_node(val);
                self.update_node_height();
            } else if val > self.as_ref().unwrap().borrow_mut().value {
                self.as_ref().unwrap().borrow_mut().right.insert_node(val);
                self.update_node_height();
            } else {
                println!("Node exists!")
            }
        }

        if self.get_balance_factor() > 1 {
            if self.as_ref().unwrap().borrow_mut().left.get_balance_factor() >= 0 {
                self.right_rotate();
            } else if self.as_ref().unwrap().borrow_mut().left.get_balance_factor() < 0 {
                self.left_right_rotate();
            }
        }
        if self.get_balance_factor() < -1 {
            if self.as_ref().unwrap().borrow_mut().right.get_balance_factor() <= 0 {
                self.left_rotate();
            } else if self.as_ref().unwrap().borrow_mut().right.get_balance_factor() > 0 {
                self.right_left_rotate();
            }
        }
    }

    fn delete_node(&mut self, val: T) {
        match self {
            None => {
                println!("No node to delete");
            }
            Some(node) => {
                if val == node.borrow_mut().value {
                    if node.borrow_mut().left.is_none() {
                        if node.borrow_mut().right.is_none() {
                            let mut right = node.borrow_mut().right.take();
                            swap(self, &mut right);
                            right = None;
                            self.update_node_height();
                        } else if node.borrow_mut().right.is_some() {
                            let mut right = node.borrow_mut().right.take();
                            swap(self, &mut right);
                            right = None;
                            self.update_node_height();
                        }
                    } else if node.borrow_mut().left.is_some() {
                        if node.borrow_mut().right.is_none() {
                            let mut left = node.borrow_mut().left.take();
                            swap(self, &mut left);
                            left = None;
                            self.update_node_height();
                        } else if node.borrow_mut().right.is_some() {
                            let mut minimum = node.borrow_mut().right.find_minimum_node().take();
                            node.borrow_mut().right.update_node_height();
                            let mut left_side = node.borrow_mut().left.take();
                            let mut right_side = node.borrow_mut().right.take();
                            swap(self, &mut minimum);
                            self.as_ref().unwrap().borrow_mut().left = left_side;
                            self.as_ref().unwrap().borrow_mut().right = right_side;
                            self.update_node_height();
                            minimum = None;
                        }
                    }
                } else if val < node.borrow_mut().value {
                    node.borrow_mut().left.delete_node(val);
                    self.update_node_height();
                } else {
                    node.borrow_mut().right.delete_node(val);
                    self.update_node_height();
                }

                if self.get_balance_factor() > 1 {
                    if self.as_ref().unwrap().borrow_mut().left.get_balance_factor() >= 0 {
                        self.right_rotate();
                    } else if self.as_ref().unwrap().borrow_mut().left.get_balance_factor() < 0 {
                        self.left_right_rotate();
                    }
                }
                if self.get_balance_factor() < -1 {
                    if self.as_ref().unwrap().borrow_mut().right.get_balance_factor() <= 0 {
                        self.left_rotate();
                    } else if self.as_ref().unwrap().borrow_mut().right.get_balance_factor() > 0 {
                        self.right_left_rotate();
                    }
                }

                self.update_node_height();
            }
        }
    }

    fn find_minimum_node(&mut self) -> Node<T> {
        // let mut res = Vec::new();
        if self.as_ref().unwrap().borrow_mut().left.is_none() {
            let mut node = self.clone();
            *self = None;
            return node;
        } else {
            self.as_ref().unwrap().borrow_mut().left.find_minimum_node()
        }
        // res
    }

    fn update_node_height(&mut self) {
        match self {
            None => {}
            Some(_) => {
                let node = self.as_ref().unwrap().borrow_mut();
                let height = max(node.left.get_node_height(), node.right.get_node_height()) + 1;
                std::mem::drop(node);
                self.as_ref().unwrap().borrow_mut().height = height;
            }
        }
    }

    fn get_node_height(&self) -> i32 {
        match self {
            None => 0,
            Some(node) => node.as_ref().borrow().height
        }
    }

    fn get_tree_height(&self) -> i32 {
        match self {
            None => 0,
            Some(_) => {
                self.as_ref().unwrap().borrow_mut().height
            }
        }
    }

    fn get_balance_factor(&self) -> i32 {
        match self {
            None => 0,
            Some(node) => node.as_ref().borrow().left.get_node_height() - node.as_ref().borrow().right.get_node_height()
        }
    }

    fn right_rotate(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                let left_node = &mut root.as_ref().borrow_mut().left.take();
                match left_node {
                    None => unreachable!(),
                    Some(node) => {
                        swap(&mut root.as_ref().borrow_mut().left, &mut node.as_ref().borrow_mut().right);
                        self.update_node_height();
                        swap(self, &mut node.as_ref().borrow_mut().right);
                        swap(self, left_node);
                        self.update_node_height();
                    }
                }
            }
        }
    }

    fn left_rotate(&mut self) {
        match self {
            None => unreachable!(),
            Some(root) => {
                let right_node = &mut root.as_ref().borrow_mut().right.take();
                match right_node {
                    None => unreachable!(),
                    Some(node) => {
                        swap(&mut root.as_ref().borrow_mut().right, &mut node.as_ref().borrow_mut().left);
                        self.update_node_height();
                        swap(self, &mut node.as_ref().borrow_mut().left);
                        swap(self, right_node);
                        self.update_node_height();
                    }
                }
            }
        }
    }

    fn left_right_rotate(&mut self) {
        match self {
            None => unreachable!(),
            Some(node) => {
                node.as_ref().borrow_mut().left.left_rotate();
                self.right_rotate();
            }
        }
    }

    fn right_left_rotate(&mut self) {
        match self {
            None => unreachable!(),
            Some(node) => {
                node.as_ref().borrow_mut().right.right_rotate();
                self.left_rotate();
            }
        }
    }

    fn inorder(&self, vec: &mut Vec<T>) {
        if let Some(node) = self {
            node.borrow_mut().left.inorder(vec);
            vec.push(node.borrow_mut().value);
            node.borrow_mut().right.inorder(vec);
        }
    }

    fn get_inorder(&self) -> Vec<T> {
        let mut res = Vec::new();
        self.inorder(&mut res);
        res
    }

    fn count_leaves(&self) -> i32 {
        let mut leaves = 0;
        match self {
            None => leaves = 0,
            Some(_) => {
                if self.as_ref().unwrap().borrow_mut().left.is_none() {
                    if self.as_ref().unwrap().borrow_mut().right.is_none() {
                        leaves = 1;
                    }
                } else {
                    let a = self.as_ref().unwrap().borrow_mut().left.count_leaves();
                    let b = self.as_ref().unwrap().borrow_mut().right.count_leaves();
                    leaves = a + b;
                }
            }
        }
        leaves
    }

    fn is_empty(&self) -> bool {
        match self {
            None => true,
            Some(_) => false,
        }
    }

    fn print(&self, prefix: &String) {
        let fix2 = prefix.clone();
        match self {
            None => {}
            Some(_) => {
                let mut current_node = self.as_ref().unwrap().borrow_mut();
                let mut node_value = current_node.value;
                println!("{:?} {:?}", prefix, node_value);
                std::mem::drop(current_node);
                let mut this_round_prefix = prefix.to_owned() + &fix2;
                self.as_ref().unwrap().borrow_mut().left.print(&this_round_prefix);
                self.as_ref().unwrap().borrow_mut().right.print(&this_round_prefix);
            }
        }
    }

    fn pretty_print(&self) {
        let prefix = String::from("->");
        self.print(&prefix);
    }

    fn is_exist(&self, val: T) -> bool {
        match self {
            None => false,
            Some(node) => {
                if node.borrow_mut().value == val {
                    return true;
                } else if val < node.borrow_mut().value {
                    node.borrow_mut().left.is_exist(val)
                } else {
                    node.borrow_mut().right.is_exist(val)
                }
            }
        }
    }
}
