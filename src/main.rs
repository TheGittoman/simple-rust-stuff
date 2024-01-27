use rand::{self, random, Rng};
use std::{array, cmp::Ordering, io, string};

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

fn main() {
    tehtävä();
}
