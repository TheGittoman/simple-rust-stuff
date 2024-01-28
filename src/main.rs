use core::num;
use rand::{self, random, Rng};
use std::{
    array,
    cmp::Ordering,
    io::{self, stdout, Write},
    string,
};

fn main() {}

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
