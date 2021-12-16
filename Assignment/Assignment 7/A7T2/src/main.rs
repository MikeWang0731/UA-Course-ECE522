use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main()
{
    let mut sample_data = Arc::new(Mutex::new(vec![1, 81, 107]));
    for i in 0..10
    {
        let data = sample_data.clone();
        thread::spawn(move || { data.lock().unwrap()[0] += i; });
    }
    thread::sleep(Duration::from_millis(50));
    println!("{:?}", sample_data);
}
