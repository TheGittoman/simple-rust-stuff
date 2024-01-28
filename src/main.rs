use core::num;
use rand::{self, random, Rng};
use std::{
    array,
    cmp::Ordering,
    io::{self, stdout, Write},
    string,
};

fn main() {
    christmas();
}

fn christmas() {
    let days = [
        "On the second day of Christmas,",
        "On the third day of Christmas,",
        "On the fourth day of Christmas,",
        "On the fifth day of Christmas,",
        "On the sixth day of Christmas,",
        "On the seventh day of Christmas,",
        "On the eighth day of Christmas,",
        "On the ninth day of Christmas,",
        "On the tenth day of Christmas,",
        "On the eleventh day of Christmas,",
        "On the twelfth day of Christmas,",
    ];
    let mut lyrics = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "And a partridge in a pear tree!",
    ];
    lyrics.reverse();

    let mut counter = 0;
    println!("On the first day of Christmas,");
    println!("my true love gave to me",);
    println!("A partridge in a pear tree");
    println!("");

    for index in days {
        println!("{index}");
        println!("my true love gave to me",);
        counter = counter + 1;
        for i in (0..counter + 1).rev() {
            println!("{}", lyrics[i]);
        }
        println!("");
    }
}

// 0 1 1 2 3 5 8 13 21 34
fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn temperature_converter() {
    println!("This is a temperature converter!");
    loop {
        let mut temperature = String::new();
        print!("Give a temperature and f or c at the end: ");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Input something!");
        io::stdout().flush().unwrap();

        let temperature = temperature.trim();

        let mut negative = 1.0;
        if temperature.contains("-") {
            negative = negative * -1.0;
        }

        let t: String = temperature.chars().filter(|c| c.is_digit(10)).collect();
        let t: f32 = match t.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if temperature.find("f").is_some() == true {
            println!("temperature in C is: {}", (t * negative - 32.0) * 5.0 / 9.0);
        } else if temperature.find("c").is_some() == true {
            println!(
                "temperature in F is: {}",
                (t * negative * (9.0 / 5.0)) + 32.0
            );
        } else {
            println!("No unit specified quitting!");
            break;
        }
    }
}

fn generate_random(limit: u32) -> u32 {
    rand::thread_rng().gen_range(0..=limit)
}

fn tehtävä() {
    println!("This is a guessing game");

    let secret_number: u32 = generate_random(100);

    loop {
        println!("Please input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
