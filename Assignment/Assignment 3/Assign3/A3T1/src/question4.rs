use super::*;
use rand::*;

#[test]
pub fn test_basic_roots() {
    // y = x^2 + 6x - 7
    let a = 1.0;
    let b = 6.0;
    let c = -7.0;
    assert_eq!(calculator::get_roots(a, b, c), (1.0, -7.0));
}
#[test]
pub fn test_single_root() {
    // y = x^2
    let a = 1.0;
    let b = 0.0;
    let c = 0.0;
    assert_eq!(calculator::get_roots(a, b, c), (0.0, 0.0));
}
#[test]
pub fn test_random_solvable_quadratic() {
    let a = rand::thread_rng().gen_range(0.0..2.0);
    let b = rand::thread_rng().gen_range(20.0..100.0);
    let c = rand::thread_rng().gen_range(0.0..10.0);
    let res = calculator::get_roots(a,b,c);
    assert_eq!((a*res.0*res.0+b*res.0+c).trunc(),0.0);
}
#[test]
#[should_panic]
pub fn test_random_non_solvable_quadratic() {
    let a = rand::thread_rng().gen_range(2.0..10.0);
    let b = rand::thread_rng().gen_range(0.0..5.0);
    let c = rand::thread_rng().gen_range(10.0..20.0);
    assert_eq!(calculator::get_roots(a, b, c), (0.0, 0.0));
}
