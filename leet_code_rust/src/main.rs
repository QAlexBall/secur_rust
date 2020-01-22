mod three_sum;
use three_sum::three_sum;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res = three_sum(nums);
    println!("Hello, world! {:?}", res);
}
