use std::cell::*;

enum Level {
    Low,
    Medium,
    High,
}


struct Task {
    id: Cell<u8>,  // Changed here
    level: Level,
}

fn main() {
    let task = Task {
        id: Cell::new(10),  // Changed here
        level: Level::High,
    };

    
    task.id.set(100);  // Changed here
    println!("Task with ID: {:?}", task.id);
}
