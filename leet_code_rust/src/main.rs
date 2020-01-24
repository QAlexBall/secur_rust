extern crate clap;
use clap::{App, Arg};
mod solutions;

fn exec_solution(s_name: String, s_arg: String) {
    println!("Solution Name is: {}, {}", s_name, s_arg);
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res = solutions::three_sum::three_sum(nums);
    println!("{:?}", res);
    for vec in Some(s_arg).into_iter() {
        println!("{:?}", vec);
    }
}

fn main() {
    let matches = App::new("Leet Code For Rust.")
        .version("1.0")
        .author("ChrisZhu <zhuderenq@outlook.com>")
        .about("Command line to introduce leetcode code, and show result!")
        .arg(
            Arg::with_name("s_name")
                .long("-s_name")
                .value_name("S_NAME")
                .help("Set Solution Name.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("s_arg")
                .long("-s_arg")
                .value_name("S_ARG")
                .help("Set arguments for solution.")
                .takes_value(true),
        )
        .get_matches();

    let s_name = matches.value_of("s_name").unwrap_or("default");
    let s_arg = matches.value_of("s_arg").unwrap_or("empty");
    exec_solution(s_name.to_string(), s_arg.to_string());
}
