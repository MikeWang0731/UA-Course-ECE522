use std::io;

fn main() {
    let input = handle_input();
    let taxed_income = calculate_tax(input);
    println!("Income after tax is {}", taxed_income);
}

pub fn calculate_tax(income: i32) -> f64 {
    let mut taxed_income: f64 = 0.0;
    let income = income as f64;
    if income >= 0.0 && income < 10000.0 {
        taxed_income = income;
    } else if income >= 10000.0 && income < 50000.0 {
        taxed_income = income - income * 0.1;
    } else if income >= 50000.0 && income < 100000.0 {
        taxed_income = income - income * 0.2;
    } else if income >= 100000.0 && income < 1000000.0 {
        taxed_income = income - income * 0.3;
    } else if income >= 1000000.0 {
        taxed_income = income - income * 0.4
    }
    taxed_income
}

pub fn handle_input() -> i32 {
    println!("Please input your income");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can not read");
    let res = is_valid(&input);
    res
}

pub fn is_valid(input: &str) -> i32 {
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Error parsing input to a integer"),
    };

    if input >= 0 {
        input
    } else {
        panic!("Input is should bigger than or equal to zero!")
    }
}

#[cfg(test)]
mod tests;
