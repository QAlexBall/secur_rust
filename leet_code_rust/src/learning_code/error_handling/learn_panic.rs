use std::io;
use std::fs::{File, read_to_string};
use std::io::{ErrorKind, Read};

fn use_panic() {
//    panic!("crash and burn!");

    let f = File::open("./test_files/hello.txt");
    let f = match f {
        Ok(file) => {
            println!("open {:?} success!", file);
            file
        },
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
       if error.kind() == ErrorKind::NotFound {
           File::create("hello.txt").unwrap_or_else(|error| {
               panic!("Problem creating the file: {:?}", error);
           })
       } else {
           panic!("Problem opening the file: {:?}", error);
       }
    });

    println!("{:?}", read_username_from_file());
    println!("{:?}", read_from_file_use_question_mark());
    println!("{:?}", read_from_file_user_question_mark_v2());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello1.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file_use_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file_user_question_mark_v2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run_learn_panic() {
    println!("run learn panic");
    use_panic();
}