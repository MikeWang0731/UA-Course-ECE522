#[test]
fn compare() {
    use super::*;
    let res = check_input("8", "6");  // we assume the input is 8 and 6.
    assert!(res.0 > res.1);
}

#[test]
#[should_panic]
fn all_positive_integer() {
    use super::*;

    let res_0 = check_input("6.7", "5.4");  // we assume the input is 6.7 and 5.4.
    // Since the number should be integers, so, it will panic.
    // In other words, it will pass the test
    assert!(res_0.0 > 0 && res_0.1 > 0);

    let res_1 = check_input("-1", "5");  // we assume the input is -1 and 5.
    // Since they both should bigger than zero, so it should panic.
    // In other words, it should pass the test.
    assert!(res_1.0 > 0 && res_1.1 > 0);
}
