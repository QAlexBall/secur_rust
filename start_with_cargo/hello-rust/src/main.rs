use ferris_says::say;
use std::io::{stdout, BufWriter};

mod my_lib;
use my_lib::*;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    test_rustaceans();
    test_summary();
    test_largest();
    test_pair();
}

fn test_pair() {
    let pair = Pair {
        x: String::from("a"),
        y: String::from("b"),
    };
    let pair1 = Pair::new(1, 3);
    pair.cmp_dispaly();
    pair1.cmp_dispaly();
}

fn test_largest() {
    let num_list = vec![2, 6, 8, 19, 3, 5];
    println!("largest number is: {}", largest(&num_list));
}

fn test_summary() {
    let tweet = Tweet {
        username: String::from("chirs"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(tweet);
}

fn test_rustaceans() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
