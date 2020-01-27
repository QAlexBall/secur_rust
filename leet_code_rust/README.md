# Rust for leetcode Solution


### Usage
```shell
# list solution args 
cargo run [name_for_solutions] --help
# examples
cargo run three_sum --help
---> Description for three_sum
---> solution for thress_sum
---> nums: Vec<i32>

cargo run -- --sname three_sum --argnum 1 --sarg "[-1, 0, 1, 2, -1, -4]"
---> three_sum([-1, 0, 1, 2, -1, -4])
---> result -> [[-1, -1, 2], [-1, 0, 1]]

# if argnum larger than 1, please use "," split sarg
cargo run -- --sname two_sum --argnum 2 --sarg "1, 2"
```
