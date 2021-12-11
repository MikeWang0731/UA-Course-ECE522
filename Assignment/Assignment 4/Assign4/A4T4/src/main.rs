use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct DoubleNode {
    value: i32,
    next: Rc<RefCell<Option<DoubleNode>>>,
    prev: Rc<RefCell<Option<DoubleNode>>>,
}
fn main() {
    let node_a = DoubleNode {
        value: 100,
        next: Rc::new(RefCell::new(None)),
        prev: Rc::new(RefCell::new(None)),
    };
    let a = Rc::new(RefCell::new(Some(node_a)));    // a-rc = 1
    let node_b = DoubleNode {
        value: 1000,
        next: Rc::clone(&a),    // a-rc = 2
        prev: Rc::new(RefCell::new(None)),
    };
    let b = Rc::new(RefCell::new(Some(node_b)));    // b-rc = 1

    println!(" a is {:?}, rc count is {}", a, Rc::strong_count(&a));
    println!(" b is {:?}, rc count is {}", b, Rc::strong_count(&b));

    if let Some(ref mut x) = *a.borrow_mut() {
        (*x).prev = Rc::clone(&b);
    }
    
    println!(" a rc count is {}", Rc::strong_count(&a));
    println!(" b rc count is {}", Rc::strong_count(&b));
}
