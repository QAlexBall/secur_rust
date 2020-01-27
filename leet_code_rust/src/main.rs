extern crate clap;
use clap::{App, Arg};
mod solutions;
use solutions::solution::*;

fn main() {
    let matches = App::new("Leet Code For Rust.")
        .version("1.0")
        .author("ChrisZhu <zhuderenq@outlook.com>")
        .about("Command line to introduce leetcode code, and show result!")
        .arg(
            Arg::with_name("s_name")
                .short("-n")
                .long("-sname")
                .value_name("S_NAME")
                .help("Set Solution Name.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("arg_num")
                .short("-i")
                .long("-argnum")
                .value_name("ARG_NUM")
                .help("numbers of arguments")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("s_arg")
                .short("-a")
                .long("-sarg")
                .value_name("S_ARG")
                .help("Set arguments for solution.")
                .takes_value(true),
        )
        .get_matches();

    let s_name = matches.value_of("s_name").unwrap_or("default");
    let arg_num = matches.value_of("arg_num").unwrap_or("1");
    let s_arg = matches.value_of("s_arg").unwrap_or("empty");
    let solution = Solution::new(
        String::from(s_name),
        String::from(arg_num),
        String::from(s_arg),
    );
    let arg = Arguments {};
    println!("result is : {:?}", solution.exec(arg));
    display_solution(solution);
}
