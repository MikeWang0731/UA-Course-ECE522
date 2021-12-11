use std::io;

fn main() {
    let user_input = get_input();
    println!("the result is {}", function(user_input.0, user_input.1));
}

// define the factorial formula
pub fn function(a: i32, b: i32) -> f64 {
    let formula = (factorial(a as f64)) / (factorial(a as f64 - b as f64) * factorial(b as f64));
    formula
}

// the logic of factorial calculation
pub fn factorial(x: f64) -> f64 {
    match x {
        0.0 => 0.0,
        1.0 => 1.0,
        _ => factorial(x - 1.0) * x,
    }
}

// check whether the input is valid
pub fn check_input(a: &str, b: &str) -> (i32, i32) {
    let input_a: i32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse to a number"),
    };

    let input_b: i32 = match b.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("Can't parse to a integer number"),
    };

    if input_a > 0 && input_b > 0 {
        if input_a > input_b {
            (input_a, input_b)
        } else {
            panic!("1st number must bigger than 2nd number!")
        }
    } else {
        panic!("Both number should bigger than zero!")
    }
}

// handle the user input and return the numbers
pub fn get_input() -> (i32, i32) {
    println!("input the first number:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Can not read!");

    println!("input the second number");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Can not read!");

    let res = check_input(&input_a, &input_b);

    res
}

#[cfg(test)]
mod test;