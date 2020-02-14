fn learn_string() {
    // Create an String
    let mut s = String::new();
    s.push_str("foo");
    println!("{}", s);
    s.push_str("bar");
    println!("{}", s);

    // use macro format! to link String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // String index
    let s1 = String::from("hello");
//    let h = s1[0];
              // ^^^^^ `std::string::String` cannot be indexed by `{integer}`
    // rust can't use index to get char

    // String slice
    let hello = "Здравствуйте";
    let s = &hello[0..2];
//    let s = &hello[0..1]; Rust panic, like un-useful index.
    println!("{}", s);

    // Traversal String
    for c in "你好, world!".chars() {
        println!("{}", c);
    }

    for b in "你好, world!".bytes() {
        println!("{}", b);
    }

    // String is not easy in Rust!!!
}

pub fn run_learn_string() {
    println!("===> Start running learn String!");
    learn_string();
    println!("===> End   running learn String!\n");
}