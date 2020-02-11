pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

mod solutions;
use solutions::three_sum::three_sum;

pub fn pub_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    three_sum(nums)
}
