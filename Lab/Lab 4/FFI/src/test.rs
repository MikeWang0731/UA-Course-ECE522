#[test]
pub fn compute_eu_distance() {
    use super::*;

    let p1 = Point{
        x: 5,
        y: 6,
    };

    let p2 = Point {
        x: -7,
        y: 11,
    };

    let correct_result = 13.0;
    assert_eq!(compute_euclidean_distance(&p1, &p2), correct_result);
}

#[test]
pub fn compute_man_distance() {
    use super::*;
    let p1 = Point{
        x: 3,
        y: 0,
    };

    let p2 = Point {
        x: 2,
        y: 0,
    };

    let correct_result = 1;
    assert_eq!(compute_manhattan_distance(&p1, &p2), correct_result);
}

#[test]
pub fn compute_che_distance() {
    use super::*;
    let p1 = Point{
        x: 5,
        y: 2,
    };

    let p2 = Point {
        x: 2,
        y: 1,
    };

    let correct_result = 3;
    assert_eq!(compute_chebyshev_distance(&p1, &p2), correct_result);
}