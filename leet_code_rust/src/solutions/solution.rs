pub trait Summary {
    fn summarize(&self) -> String {
        format!("Summary for Solution")
    }
}

pub trait SExcutable<T> {
    fn exec(&self) -> T;
}
