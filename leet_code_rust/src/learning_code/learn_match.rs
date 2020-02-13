enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_if_let(num: Option<i32>) {
    println!("Input Num is {:?}", num);
    if let Some(5) = num {
        println!("Is Five!");
    } else if let Some(3) = num{
        println!("Is Three!");
    } else {
        println!("Isn't Three! or Five");
    }
}

pub fn run_learn_match() {
    println!("==========> start running learn match code!");
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{}", five.unwrap());
    println!("{:?}, {:?}, {:?}\n", five, six, none);

    test_if_let(Some(5));
    test_if_let(Some(3));
    test_if_let(Some(6));
    println!("==========>  done running learn match code!");
}