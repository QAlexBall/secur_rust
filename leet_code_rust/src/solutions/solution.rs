pub trait Summary {
    fn summarize(&self) -> String {
        format!("Summary for Solution")
    }
}

#[derive(Debug)]
pub struct Result {
    result: String,
}

impl Result {
    fn new(result: String) -> Self {
        Self { result }
    }
}

pub struct Solution {
    s_name: String,
    arg_num: String,
    s_args: String,
}

impl Solution {
    pub fn new(s_name: String, arg_num: String, s_args: String) -> Self {
        Self {
            s_name,
            arg_num,
            s_args,
        }
    }

    // exmaple solution.(three_sum:Solution (args: Arguments)) ==> Result
    pub fn exec(&self, args: Arguments) -> Result {
        let s_name = self.s_name.clone();
        let result = Result::new(s_name);
        // let result = Result.new(String::From(self.s_name(args)));
        result
    }
}

impl Summary for Solution {
    fn summarize(&self) -> String {
        format!("Summary for {}, with args {}", self.s_name, self.s_args)
    }
}

pub fn display_solution(item: impl Summary) {
    println!("@{}", item.summarize());
}

pub struct Arguments {}
