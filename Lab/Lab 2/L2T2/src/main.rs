#[derive(Debug,PartialEq)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        // if already have, skip
        if self.data == data {
            return;
        }
        // if no, find the appropriate location
        let new_node = if data < self.data { &mut self.left_child } else { &mut self.right_child };
        // Prepare to add value
        match new_node {
            // if it is not the final destination, keep recursive
            &mut Some(ref mut node) => node.insert_node(data),
            // if it is, init a node, make it to the right type we want, and then assign.
            &mut None => {
                let create_node = TreeNode { data, left_child: None, right_child: None };
                let box_node = Some(Box::new(create_node));
                *new_node = box_node;
            },
        }
    }
}


fn main() {
    let mut node_1 = TreeNode {
        data: "10",
        left_child: None,
        right_child: None
    };
    node_1.insert_node("09");
    node_1.insert_node("11");
    println!("{:?}", node_1);
}
