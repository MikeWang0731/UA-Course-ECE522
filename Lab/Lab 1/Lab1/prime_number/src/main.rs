use prime_tools::*;
use rug::{Assign, Integer};
use rand::Rng;

fn main() {
    println!("Return value: {}", function(203));
}

fn function(n: u32) -> Integer{
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate = Integer::new();
        candidate.assign(rng.gen_range(0..n));

        if is_u32_prime(candidate.to_u32().unwrap()) == true {
            return candidate;
        }
    }
}

