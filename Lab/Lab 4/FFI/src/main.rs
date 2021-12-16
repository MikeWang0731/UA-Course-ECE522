use std::cmp::max;

#[cfg(test)]
mod test;

pub struct Point {
    x: i8,
    y: i8,
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    // sqrt((x1-x2)^2 + (y1-y2)^2)
    return (((p1.x - p2.x) as f64).powi(2) + ((p1.y - p2.y) as f64).powi(2)).sqrt();
}

extern {
    pub fn abs(i: i32) -> i32;
}

pub fn compute_manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        a_abs + b_abs
    }
}

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    let x_abs = (p1.x as i32 - p2.x as i32).abs();
    let y_abs = (p1.y as i32 - p2.y as i32).abs();
    max(x_abs, y_abs)
}

pub fn compute_chebyshev_distance_c(p1: &Point, p2: &Point) -> i32 {
    unsafe {
        let x_abs = abs(p1.x as i32 - p2.x as i32);
        let y_abs = abs(p1.y as i32 - p2.y as i32);
        max(x_abs, y_abs)
    }
}

fn handle_input() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Cannot read!");
    input.trim().parse().unwrap()
}

fn main() {
    println!("===== Distance Calculator =====");
    println!("Please input the coordinate for the 1st point.");
    println!("x1: ");
    let x1: i32 = handle_input();
    println!("y1: ");
    let y1: i32 = handle_input();
    let p1 = Point {
        x: x1 as i8,
        y: y1 as i8,
    };

    println!("Please input the coordinate for the 2nd point.");
    println!("x2: ");
    let x2: i32 = handle_input();
    println!("y2: ");
    let y2: i32 = handle_input();
    let p2 = Point {
        x: x2 as i8,
        y: y2 as i8,
    };

    println!("The points you entered are: ({},{}) and ({},{})",
             p1.x, p1.y, p2.x, p2.y
    );

    loop {
        println!("\nPlease choose what kind of distance you want to get:");
        println!("1.Euclidean Distance \n2.Manhattan Distance \n3.Chebyshev Distance \n4.Exit");

        let choice: i32 = handle_input();
        match choice {
            1 => {
                println!("Euclidean Distance is: {}", compute_euclidean_distance(&p1, &p2));
            }
            2 => {
                println!("Manhattan Distance is: {}", compute_manhattan_distance(&p1, &p2));
            }
            3 => {
                println!("Chebyshev Distance is: {}", compute_chebyshev_distance(&p1, &p2));
            }
            4 => {
                break;
            }
            _ => {
                println!("Wrong instruction, try again")
            }
        };
    }
}
