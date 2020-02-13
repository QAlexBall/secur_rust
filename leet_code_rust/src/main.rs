extern crate clap;
use clap::{App, Arg};
extern crate libloading as lib;

pub mod solutions;
use solutions::three_sum::*;
mod learning_code;
use learning_code::learn_match::run_learn_match;
use learning_code::learn_collections::run_learn_collections;

fn main() {
    run_learn_match();
    run_learn_collections();
    let matches = App::new("LeetCode For Rust.")
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
    println!("{}", s_name);
    if s_name == "three_sum" {
        let solution = ThreeSum::new(
            String::from(s_name),
            String::from(arg_num),
            String::from(s_arg),
        );
        display_result(&solution);
        println!("{:?}", solution);
    }
    let add_string = |x: String| -> String { String::from(x + "a") };
    println!("{:?}", add_string(String::from("b")));
}
