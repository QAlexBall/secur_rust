use leet_code_rust;

#[test]
fn it_adds_two() {
    assert_eq!(4, leet_code_rust::add_two(2));
}

#[test]
fn three_sum_test() {
    let input = vec![-4, -5, 3, 1, 0, 2, -1];
    let result = vec![vec![-5, 2, 3], vec![-4, 1, 3], vec![-1, 0, 1]];
    assert_eq!(result, leet_code_rust::pub_three_sum(input));
}