use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
struct Bank {
    accounts: Arc<Mutex<Vec<i32>>>,
}

impl Bank {
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(i as i32);
        }
        Bank {
            accounts: Arc::new(Mutex::new(v)),
        }
    }
    fn transfer(&self, from: usize, to: usize, amount: i32) -> Result<(), ()> {
        let mut temp = self.accounts.lock().unwrap();
        let mut flag = false;
        if temp.contains(&(from as i32)) && temp.contains(&(to as i32)) {
            flag = true;
        }
        if flag == true {
            println!("Amount of ${} transferred from account id: {} to account id: {}.", amount, from, to);
            Ok(())
        } else {
            Err(())
        }
    }
}

struct Person {
    ac_id: usize,  // this person's
    buddy_id: usize,  // transfer target
}

impl Person {
    pub fn new(id: usize, b_id: usize) -> Self {
        Person {
            ac_id: id,
            buddy_id: b_id,
        }
    }
}


fn main() {
    // let account = Bank::new(20);
    // println!("{:?}", account.transfer(5, 10, 100));
    let bank_ac = Bank::new(15);
    for i in 0..16{
        let ac = bank_ac.clone();
        let person = Person::new(i, i + 1);
        let handle = thread::spawn(move || {
            ac.transfer(person.ac_id, person.buddy_id, 100);
        });
        handle.join().unwrap();
    }
}