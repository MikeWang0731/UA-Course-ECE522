use crate::AVL::{AvlTree, AvlTreeNode};
pub mod AVL;
pub mod BST;
pub mod RBTree;

pub fn run_avl_tree_example() {
    let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
    for i in vec![2, 1, 3, 5, 4, 6] {
        avl_tree.insert_node(i);
    }
    println!(" ===== Add {:?} to avl tree ===== ", vec![2, 1, 3, 5, 4, 6]);
    avl_tree.print_tree_diagram();

    println!(" ===== Print some information about the tree ===== ");
    println!("Number of leaves: {}", avl_tree.number_of_leaves());
    println!("Height of tree: {}", avl_tree.height_of_tree());
    println!("Balanced Tree? {}", AvlTree::validate_tree(&avl_tree));
    if avl_tree.is_tree_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }

    println!(" ===== Perform some traversals ===== ");
    println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
    println!("Pre Order Traverse: {:?}", avl_tree.pre_order_traverse());
    println!("Post Order Traverse: {:?}", avl_tree.post_order_traverse());

    println!(" ===== Delete {:?} from the tree =====", vec![2, 6, 8]);
    avl_tree.delete_node(2);
    let s = avl_tree.delete_node(6);
    println!("The deleted Node(6) contains {:?}", s);
    avl_tree.delete_node(8);

    println!(" ===== Check the result of above operation ===== ");
    for i in vec![1, 2, 3, 4, 5, 6] {
        println!("Does {} exist? {}", i, avl_tree.exist_or_not(i));
    };
    avl_tree.print_tree_diagram();

    println!(" ===== Add some new nodes ([7, 8]) ===== ");
    for i in vec![7, 8] {
        avl_tree.insert_node(i);
    };
    avl_tree.print_tree_diagram();

    println!(" ===== Update node value 8 -> 9 ===== ");
    avl_tree.update_node(8, 9);

    println!(" ===== let's do a in order traversal in the end ===== ");
    println!("In Order Traverse: {:?}", avl_tree.in_order_traverse());
    println!("This AVL tree has a total of {} elements.", avl_tree.total_number_elements());
}

pub fn run_rb_tree_example() {
    //add element and print the whole tree
    let mut rb_tree = RBTree::RBTree::new();
    for i in vec![1, 2, 3, 4, 5, 6] {
        rb_tree.insert_node(i);
    }
    println!(" ===== Add {:?} to red-black tree ===== ", vec![1, 2, 3, 4, 5, 6]);
    rb_tree.print_tree();
    //print basic information of rb_tree
    println!("Number of leaves: {}", rb_tree.get_number_leaves());
    println!("Height of tree: {}", rb_tree.get_height());
    //pre/in/post reversal
    println!("In Order Traverse: {:?}", rb_tree.print_in_order_traversal());
    println!("Pre Order Traverse: {:?}", rb_tree.print_pre_order_traversal());
    println!("Post Order Traverse: {:?}", rb_tree.print_post_order_traversal());
    //is empty?
    if rb_tree.is_empty() { println!("Tree is Empty") } else { println!("Tree is not empty!") }
    println!(" ===== Delete {:?} from the tree =====", vec![3, 4, 5]);
    //delete 3,4,5
    rb_tree.delete(3);
    rb_tree.delete(4);
    rb_tree.delete(5);
    println!("The deleted Node contains: 3, 4, 5");
    // node exists？
    println!(" ===== Check the result of above operation ===== ");
    for i in vec![1, 2, 3, 4, 5, 6] {
        println!("Does {} exist? {}", i, rb_tree.exist_or_not(i));
    }
    //print the rb_tree after doing delete operation
    rb_tree.print_tree();
    // add new nodes -> rb_tree:[1, 2, 6, 7, 8]
    println!(" ===== Add some new nodes ([7, 8]) ===== ");
    for i in vec![7, 8] {
        rb_tree.insert_node(i);
    }
    //print tree after doing insert operation
    rb_tree.print_tree();
    println!(" ===== Update node value 2 -> 3 ===== ");
    rb_tree.update_node(2, 3);
    println!(" ===== let's do a in order traversal in the end ===== ");
    println!("In Order Traverse: {:?}", rb_tree.print_in_order_traversal());
    println!("This RBTree has a total of {} elements.", rb_tree.total_number_elements());
}