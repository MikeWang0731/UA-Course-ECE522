use super::*;
use rand::*;
use hamcrest::prelude::*;

#[test]
pub fn test_random_positive_square_root() {
    // let rng = rand::thread_rng().gen::<f64>();
    let rng = rand::thread_rng().gen_range(0.0..100.0);
    assert_that!(calculator::get_squre_root(rng), equal_to((rng as f64).sqrt()));
}
#[test]
#[should_panic]
pub fn test_random_negitive_square_root() {
    let rng = rand::thread_rng().gen_range(-100.0..0.0);
    assert_that!(calculator::get_squre_root(rng), equal_to((rng as f64).sqrt()));
}
#[test]
pub fn test_square_root_of_zero() {
    let x = 0.0;
    assert_eq!(calculator::get_squre_root(x), 0.0);
    assert_that!(calculator::get_squre_root(x), equal_to(0.0));
}

#[test]
pub fn test_square_root_of_one() {
    let x = 1.0;
    assert_eq!(calculator::get_squre_root(x), 1.0);
    assert_that!(calculator::get_squre_root(x), equal_to(1.0));
}
