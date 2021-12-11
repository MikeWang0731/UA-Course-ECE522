use super::*;
use rand::prelude::*;

#[test]
pub fn basic_multiply() {
    let x = 4;
    let y = 2;
    assert_eq!(calculator::multiply(x, y), 8);
}

#[test]
pub fn multiply_negative_number() {
    let x = -4;
    let y = -2;
    assert_eq!(calculator::multiply(x, y), 8)
}

#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    if rng.gen() {
        // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen();
        assert_eq!(calculator::multiply(x, y), x * y);
    }
}

#[test]
pub fn basic_divide() {
    let x = 2;
    let y = 2;
    assert_eq!(calculator::divide(x, y), 1);
}

#[test]
pub fn divide_negative_number() {
    let x = -2;
    let y = -2;
    assert_eq!(calculator::divide(x, y), 1);
}
#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    if rng.gen() {
        // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y: f64 = rng.gen();
        assert_eq!(calculator::divide(x, y), x / y);
    }
}
