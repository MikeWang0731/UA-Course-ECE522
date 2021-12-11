use ECE522_project::run_avl_tree_example;
use ECE522_project::run_rb_tree_example;
use crate::AVL::{AvlTree, AvlTreeNode};
use RBTree::*;

mod RBTree;
mod AVL;

fn main() {
    run_command_line_app();
}

fn handle_input() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Cannot read!");
    let res = input.trim().parse::<i32>().unwrap_or(100);
    res
}

fn input_to_vec() -> Result<Vec<i32>, ()> {
    let mut numbers = String::new();
    std::io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let numbers = numbers.split_whitespace();
    let mut vec: Vec<i32> = Vec::new();
    for i in numbers {
        match i.parse() {
            Ok(i) => vec.push(i),
            Err(_) => println!("Please replace '{}' with a integer!", i),
        }
    }
    Ok(vec)
}

fn input_to_u32_vec() -> Result<Vec<u32>, ()> {
    let mut numbers = String::new();
    std::io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");
    let numbers = numbers.split_whitespace();
    let mut vec: Vec<u32> = Vec::new();
    for i in numbers {
        match i.parse() {
            Ok(i) => vec.push(i),
            Err(_) => println!("Please replace '{}' with a no-sign integer!", i),
        }
    }
    Ok(vec)
}

// command line instruction list
fn instruction_list() {
    println!(
        "1. cargo run avl: Go to AVL tree interface\n\
         2. cargo run rb: Go to Red-Black tree interface\n\
         3. cargo run prebuild: Run pre-build AVL and RB tree examples
         "
    )
}

fn avl_help_list() {
    println!("=========== AVL HELP MANUAL ===========");
    println!("0 - Exit\n\
              1 - Insert: insert a node/some nodes to the avl tree\n\
              2 - Delete: delete a node/some nodes from the avl tree\n\
              3 - Leaves: count the number of leaves in this avl tree\n\
              4 - Height: check the height of this avl tree\n\
              5 - In Order Traversal: print the in-order traversal of the avl tree\n\
              6 - Pre Order Traversal: print the pre-order traversal of the avl tree\n\
              7 - Post Order Traversal: print the post-order traversal of the avl tree\n\
              8 - Empty Or Not: check it is empty or not\n\
              9 - Print: print this tree\n\
              10 - Update: Update the value of a specific node (replace A with B)\n\
              11 - Exist Or Not: Check whether a value exists\n\
              12 - Validate: Check whether it is a balanced tree\n\
              13 - Total Number: Total number of elements");
    println!("=======================================");
}

fn rb_help_list() {
    println!("=========== RBTree HELP MANUAL ===========");
    println!("0 - Exit\n\
              1 - Insert: insert a node/some nodes to the avl tree\n\
              2 - Delete: delete a node/some nodes from the avl tree\n\
              3 - Leaves: count the number of leaves in this avl tree\n\
              4 - Height: check the height of this avl tree\n\
              5 - In Order Traversal: print the in-order traversal of the avl tree\n\
              6 - Pre Order Traversal: print the pre-order traversal of the avl tree\n\
              7 - Post Order Traversal: print the post-order traversal of the avl tree\n\
              8 - Empty Or Not: check it is empty or not\n\
              9 - Print: print this tree\n\
              10 - Update: Update the value of a specific node (replace A with B)\n\
              11 - Exist Or Not: Check whether a value exists\n\
              12 - Total Number: Total number of elements");
    println!("=======================================");
}

fn run_command_line_app() {
    let args: Vec<String> = std::env::args().collect();
    let length = args.len();

    if length < 2 {
        instruction_list();
    } else {
        let keyword = &args[1];  // cargo run[0] xxx[1] ...
        match keyword.as_str() {
            "avl" => {
                if length != 2 {
                    eprintln!("Wrong number of arguments, please follow [cargo run avl]");
                    // std::process::exit(1);
                } else {
                    let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
                    loop {
                        avl_help_list();
                        println!("Please input your choice: ");
                        let user_choice = handle_input();
                        match user_choice {
                            0 => { break; }
                            1 => {
                                println!("Please input what kind of value you want to add. Separate by one whitespace.\n\
                                e.g.1 2 3 4 5");
                                let input = input_to_vec().unwrap();
                                let mut output = Vec::new();
                                for i in input.clone() {
                                    if !avl_tree.exist_or_not(i) {
                                        avl_tree.insert_node(i);
                                        output.push(i);
                                    } else {
                                        println!("INSERT FAILED: Node({:?}) already exists!", i);
                                    }
                                }
                                if output.len() != 0 {
                                    println!("Insert {:?} successfully.", output);
                                }
                            }
                            2 => {
                                println!("Current tree contains {:?}", avl_tree.in_order_traverse());
                                println!("Please input what kind of value you want to delete. Separate by one whitespace.\n\
                                e.g.1 2 3 4 5");
                                let input = input_to_vec().unwrap();
                                for i in input.clone() {
                                    avl_tree.delete_node(i);
                                }
                            }
                            3 => println!("Number of leaves: {}", avl_tree.number_of_leaves()),
                            4 => println!("Height of tree: {}", avl_tree.height_of_tree()),
                            5 => println!("In Order Traverse: {:?}", avl_tree.in_order_traverse()),
                            6 => println!("Pre Order Traverse: {:?}", avl_tree.pre_order_traverse()),
                            7 => println!("Post Order Traverse: {:?}", avl_tree.post_order_traverse()),
                            8 => {
                                if avl_tree.is_tree_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }
                            }
                            9 => avl_tree.print_tree_diagram(),
                            10 => {
                                println!("Please input the node you want to update. Separate by one whitespace\n\
                                e.g.1 2(replace 1 with 2)");
                                let input = input_to_vec().unwrap();
                                if input.len() != 2 {
                                    eprintln!("Wrong number of input. Try again...")
                                } else {
                                    avl_tree.update_node(input.get(0).unwrap().to_owned(), input.get(1).unwrap().to_owned());
                                }
                            }
                            11 => {
                                println!("Please input the node/nodes you want to check. Separate by one whitespace.");
                                let input = input_to_vec().unwrap();
                                if input.len() == 0 {
                                    println!("Numbers of node can not be zero!")
                                } else {
                                    for i in input {
                                        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
                                    }
                                }
                            }
                            12 => {
                                println!("Balanced Tree? {}", avl_tree.validate_tree());
                            }
                            13 => {
                                println!("This AVL tree has a total of {} elements.", avl_tree.total_number_elements());
                            }
                            _ => println!("Wrong input! Input should be a number from the list, please try again..."),
                        }
                        std::thread::sleep(std::time::Duration::from_millis(800));
                    }
                    println!("Thank you! Hope to see you again!");
                };
            }
            "rb" => {
                if length != 2 {
                    eprintln!("Wrong number of arguments, please follow [cargo run rb]");
                    // std::process::exit(1);
                } else {
                    let mut rb_tree: RBTree::RBTree = RBTree::RBTree::new();
                    loop {
                        rb_help_list();
                        println!("Please input your choice: ");
                        let user_choice = handle_input();
                        match user_choice {
                            0 => { break; }
                            1 => {
                                println!("Please input what kind of value you want to add. Separate by one whitespace.\n\
                                e.g.1 2 3 4 5");
                                let input: Vec<u32> = input_to_u32_vec().unwrap();

                                for i in input.clone() {
                                    if !rb_tree.exist_or_not(i) {
                                        rb_tree.insert_node(i);
                                        println!("Insert {:?} successfully.", i);
                                    }else{
                                        println!("INSERT FAILED: Node({:?}) already exists!", i);
                                    }

                                }
                            }
                            2 => {
                                println!("Current tree contains {:?}", rb_tree.print_in_order_traversal());
                                println!("Please input what kind of value you want to delete. Separate by one whitespace.\n\
                                e.g.1 2 3 4 5");
                                let input: Vec<u32> = input_to_u32_vec().unwrap();
                                for i in input.clone() {
                                    if rb_tree.exist_or_not(i){
                                        rb_tree.delete(i);
                                        println!("Delete {:?} successfully",i);
                                    }else{
                                        println!("Delete FAILED: Node({:?}) doesn't exist!", i);
                                    }
                                }
                            }
                            3 => println!("Number of leaves: {}", rb_tree.get_number_leaves()),
                            4 => println!("Height of tree: {}", rb_tree.get_height()),
                            5 => println!("In Order Traverse: {:?}", rb_tree.print_in_order_traversal()),
                            6 => println!("Pre Order Traverse: {:?}", rb_tree.print_pre_order_traversal()),
                            7 => println!("Post Order Traverse: {:?}", rb_tree.print_post_order_traversal()),
                            8 => {
                                if rb_tree.is_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }
                            }
                            9 => rb_tree.print_tree(),
                            10 => {
                                println!("Please input the node you want to update. Separate by one whitespace\n\
                                e.g.1 2(replace 1 with 2)");
                                let input = input_to_u32_vec().unwrap();
                                if input.len() != 2 {
                                    eprintln!("Wrong number of input. Try again...")
                                } else {
                                    rb_tree.update_node(input.get(0).unwrap().to_owned(), input.get(1).unwrap().to_owned());
                                }
                            }
                            11 => {
                                println!("Please input the node/nodes you want to check. Separate by one whitespace.");
                                let input = input_to_u32_vec().unwrap();
                                if input.len() == 0 {
                                    println!("Numbers of node can not be zero!")
                                } else {
                                    for i in input {
                                        println!("Does {} exist? {}", i, rb_tree.exist_or_not(i));
                                    }
                                }
                            }
                            12 => {
                                println!("This RBTree has a total of {} elements.", rb_tree.total_number_elements());
                            }
                            _ => println!("Wrong input! Input should be a number from the list, please try again..."),
                        }
                    }
                    println!("Thank you! Hope to see you again!");
                }
            }

            "prebuild" => {
                println!("Please choose what kind of example you want to run?\n\
                1 - AVL tree\n\
                2 - Red-Black tree");
                let input = handle_input();
                if input == 1 {
                    run_avl_tree_example()
                } else if input == 2 {
                    run_rb_tree_example()
                } else {
                    println!("Wrong input, please try again...");
                }
            }
            _ => println!("Wrong command instruction, please try again!"),
        };
    }
}
