use crate::avl_tree::Avl;

mod avl_tree;
fn main() {
    let mut node = avl_tree::Node::generate_new_node(7);
    for i in vec![6,2,1,8,9,3] {
        node.insert_node(i)
    };
    println!("Leaves: {}", node.count_leaves());
    println!("Height: {}", node.get_tree_height());
    println!("Inorder: {:?}", node.get_inorder());
    println!("Empty? {}", node.is_empty());
    node.pretty_print();

    for i in vec![2,8,0,12,9,3] {
        println!("Does {} exist? {}", i, node.is_exist(i));
    }

    node.delete_node(2);
    node.delete_node(8);
    node.delete_node(3);
    println!("Inorder: {:?}", node.get_inorder());
    node.pretty_print();
    println!("Leaves: {}", node.count_leaves());
    println!("Height: {}", node.get_tree_height());
    println!("Empty? {}", node.is_empty());
}
