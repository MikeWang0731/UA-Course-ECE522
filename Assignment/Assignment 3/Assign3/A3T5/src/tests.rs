#[test]
#[should_panic]
fn is_negative() {
    use super::*;
    let input = is_valid(&"-24500");
    assert!(input > 0);
}

#[test]
#[should_panic]
fn is_not_integer() {
    use super::*;
    let input = is_valid(&"4500.35");
    assert!(input > 0);
}
