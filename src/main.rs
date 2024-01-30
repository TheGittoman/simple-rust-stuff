use rand::distributions::DistIter;

fn main() {
    let coin = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alabama);
    println!("Value of coin in cets: {}", value_in_cents(coin));
    println!("Value of coin in cets: {}", value_in_cents(coin2));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let dice_roll = 9;
    match dice_roll {
        3 => println!("You get 3"),
        6 => println!("You get 6"),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configurad to be {}", max);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
