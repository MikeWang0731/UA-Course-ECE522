use std::ops::*;

pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

pub fn multiply<T: Mul>(x: T, y: T) -> T::Output {
    x * y
}

pub fn divide<T: Div>(x: T, y: T) -> T::Output {
    x / y
}

pub fn get_squre_root(x: f64) -> f64 {
    if x < 0.0 {
        panic!("Negative numbers don't have real square roots!");
    }
    x.sqrt()
}

pub fn get_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    // quadratic equations: ax^2+bx+c=0
    // (-b + sqrt(b^2-4ac))/(2a) or (-b - sqrt(b^2-4ac))/(2a)
    let delta = b * b - 4.0 * a * c;
    if delta >= 0.0 {
        let root_one = (-b + delta.sqrt()) / (2.0 * a);
        let root_two = (-b - delta.sqrt()) / (2.0 * a);
        (root_one, root_two)
    } else {
        panic!("There is no root for this equation!");
        println!("There is no root for this equation!");
    }
}
