fn use_vector() {
    let v: Vec<i32> = Vec::new(); // create new None Vector to save i32
    let v = vec![1, 2, 3]; // create an vector which has init value

    // update vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // read from vector
    let third: &i32 = &v[2];
    println!("The third elements is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // if we get an immutable element from vector, can't push new element to this vector
    /*
    v.push(6);
    println!("The third element is: {}", third);
      --> src/learning_code/learn_collections/learn_vector.rs:22:5
   |
13 |     let third: &i32 = &v[2];
   |                        - immutable borrow occurs here
...
22 |     v.push(6);
   |     ^^^^^^^^^ mutable borrow occurs here
    */

    // Traversal vector

    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50; // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
        println!("{}", i);
    }

    // use enum to save more type
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("yellow")),
        SpreadsheetCell::Float(3.1415926),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}


pub fn run_learn_vector() {
    println!("===> Start running learn vector!");
    use_vector();
    println!("===> End   running learn vector!\n");
}