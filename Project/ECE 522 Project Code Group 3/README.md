# AVL Tree Design Document

> This AVL tree is part of a tree data structure project by Zhaoyi Wang (Mike), Zihao Wang (Bluce) and Shuo Wang (Tina). The goal of this project is to write AVL tree and Red Black tree using `Rust` language. 

## Part 1: Brief Overview

Except the following basic features marked by ✅, we add some additional features to this tree marked by 🌟.

- ✅ Insert / Delete / Count Leaves / Calculate Height / In-order Traversal / Check Empty / Print Tree
- 🌟 Pre-order Traversal / Post-order Traversal / Check Element Existence / Update Node / Validate Tree / Total Number of Elements


## Part 2: Current Shortcomings


❌ Up to this stage, if users want to test in the command line, then their input can only be of numeric type (e.g., `1,2,3,4,5`). This version does not support users using character or string types in the command line interface for now (e.g., `'a','b','c'`). *However, users can import the file and use the interface we defined while writing their code and this will not have any restrictions.*


## Part 3: User Manual


This AVL Tree implementation based on `Box<>` and `Option<>`.

A tree type defined like the following:

```rust
pub type AvlTreeNode<T> = Option<Box<TreeNode<T>>>;
```

#### Quick Start

Before we start, copy the `AVL.rs` file to your root path (`~/project_name/src`), and add the following line to `main.rs`.

```rust
mod AVL;
use crate::AVL::{AvlTree, AvlTreeNode};
```

First, we need to define an empty tree, and make sure it is mutable.

```rust
let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
```

Second, we can add some value to this tree. Here I will use for loop.

```rust
for i in vec![5, 3, 1, 4, 2] {
        avl_tree.insert_node(i);
    }
```

Then, print the tree and validate it.

```rust
avl_tree.print_tree_diagram();
println!("Balanced Tree? {}", avl_tree.validate_tree());
```

Next, we can calculate the leaves and count the height of the tree.

```rust
println!("Number of leaves: {}", avl_tree.number_of_leaves());
println!("Height of tree: {}", avl_tree.height_of_tree());
```

Also, if you need, you can do three types of traversal to your tree.

```rust
println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
println!("Pre Order Traverse: {:?}", avl_tree.pre_order_traverse());
println!("Post Order Traverse: {:?}", avl_tree.post_order_traverse());
```

Don't forget to check whether it is empty if you are not sure about whether the above operation was successful.

```rust
if avl_tree.is_tree_empty() { 
  		println!("Tree is Empty") 
	} else { 
  		println!("Tree is not empty!") 
}
```

After that, we can delete some node.

```rust
avl_tree.delete_node(1);
```

You can get the feedback at the same time, if you want.

```rust
let res = avl_tree.delete_node(2);
println!("The deleted Node(2) contains {:?}", res);
```

To check the result of the `insert_node()` and `delete_node()` operation, you can do the following to check the existence of a specific node.

```rust
for i in vec![4, 6, 5] {
  			// Check the existence of Node(4), Node(5) and Node (6)
        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
    }
```

By the way, you can update a node just like the way you want to update a info in your database.

```rust
avl_tree.update_node(1, 2);  // 1 is the OLD one, 2 is the NEW one
```

Finally, let us do in-order traversal again and print the final tree.

```rust
println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
avl_tree.print_tree_diagram();
```

And see how many elements we have now.

```rust
println!("This AVL tree has a total of {} elements.", avl_tree.total_number_elements());
```

#### Public Interface

```rust
fn insert_node(&mut self, val: T);
// insert a node
fn delete_node(&mut self, val: T) -> Self;
// delete a node
fn validate_tree(&self) -> bool;
// balanced or not?
fn is_tree_empty(&self) -> bool;
// empty or not?
fn height_of_tree(&self) -> i32;
// get the height of this tree
fn number_of_leaves(&self) -> i32;
// how many leaves 
fn in_order_traverse(&mut self) -> Vec<T>;
// In-order traverse
fn pre_order_traverse(&mut self) -> Vec<T>;
// Pre-order traverse
fn post_order_traverse(&mut self) -> Vec<T>;
// Post-order traverse
fn print_tree_diagram(&mut self);
// Nicely print the tree
fn exist_or_not(&self, val: T) -> bool;
// Check whether a value exists
fn generate_empty_tree() -> Self;
// generate a new empty tree
fn update_node(&mut self, old: T, new: T);
// update a node
fn total_number_elements(&mut self) -> i32;
// count total number of elements
```

------

#  RBTree Design Document

> This Red-black tree is part of a tree data structure project by Zhaoyi Wang (Mike), Zihao Wang (Bluce) and Shuo Wang (Tina). The goal of this project is to write AVL tree and Red Black tree using `Rust` language.


## Part 1: Major Innovations


#### Brief Overview

Except the following basic features marked by ✅, we add some additional features to this tree marked by 🌟.

- ✅ Insert / Delete / Count Leaves / Calculate Height / In-order Traversal / Check Empty / Print Tree
- 🌟 Pre-order Traversal / Post-order Traversal / Check Element Existence / Update Node/Total number of elements in a tree

## Part 2: Current Shortcomings


❌ Up to this stage, if users want to test in the command line, then their input can only be of numeric type (e.g., `1,2,3,4,5`). This version does not support users using character or string types in the command line interface for now (e.g., `'a','b','c'`). *However, users can import the file and use the interface we defined while writing their code and this will not have any restrictions.*


## Part 3: Functions


The functions of Red-black are as follow:

1. ```rust
   pub fn insert_node(&mut self, val: u32) -> bool;
   ```

   Test whether the node is inserted successfully.

2. ```rust
   pub fn get_number_leaves(&self) -> u32;
   ```

   Return the number of leaves in a tree.

3. ```rust
   pub fn get_height(&self) -> u32;
   ```

   Return the height of a tree.

4. ```rust
   pub fn is_empty(&self) -> bool;
   ```

   Test whether the tree is empty.

5. ```rust
   pub fn exist_or_not(&mut self,val:u32) -> bool;
   ```

   Test whether the value exists in the tree.

6. ```rust
   pub fn update_node(&mut self,old_val:u32,new_val:u32);
   ```

   Update the tree using new value to replace old value.

7. ```rust
   pub fn delete(&mut self, val: u32) -> Result<(), String>;
   ```

   Delete the node with value in the tree, return the delete result either success(Ok) or failure (Err).

8. ```rust
   pub fn print_in_order_traversal(&self) -> Vec<u32>;
   ```

   Return the vector based on in-order traversal.

9. ```rust
   pub fn print_pre_order_traversal(&self) -> Vec<u32>;
   ```

   Return the vector based on pre-order traversal.

10. ```rust
    pub fn print_post_order_traversal(&self) -> Vec<u32>;
    ```

    Return the vector based on post-order traversal.

11. ```rust
    pub fn print_tree(&self)
    ```

    Print the tree in format<L/R> <Key>:<Color>.

12. ```rust
    pub fn total_number_elements(&self) ->i32
    ```

    Print total number of elements in a tree.

    

## Part 4: User Manual


First, we need to define an empty tree and insert some values into this tree.

```rust
let mut rb_tree = RBTree::RBTree::new();
for i in vec![1,2,3,4,5,6] {
    rb_tree.insert_node(i);
}
```

Then, we can print the tree, print the number of leaves, print the height and print the tree based on in-order, pre-order or post-order traversal.

```rust
//print the tree
rb_tree.print_tree();
//print the number of leaves of the tree
println!("Number of leaves: {}", rb_tree.get_number_leaves());
//print the height of the tree
println!("Height of tree: {}", rb_tree.get_height());
//print pre/in/post reversal
println!("In Order Traverse: {:?}", rb_tree.print_in_order_traversal());
println!("Pre Order Traverse: {:?}", rb_tree.print_pre_order_traversal());
println!("Post Order Traverse: {:?}",rb_tree.print_post_order_traversal());
```

Don't forget to check whether it is empty when you not sure about your tree.

```rust
if rb_tree.is_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }
```

After that, we can delete some node.

```rust
rb_tree.delete(3);
rb_tree.delete(4);
rb_tree.delete(5);
```

To check the result of the `insert_node()` and `delete_node()` operation, you can do the following to check the existence of a specific node.

```rust
for i in vec![1,2,3,4,5,6] {
    println!("Does {} exist? {}", i, rb_tree.exist_or_not(i));
}
```

By the way, you can update a node just like the way you want to update a info in your database.

```rust
rb_tree.update_node(2, 3);
```
