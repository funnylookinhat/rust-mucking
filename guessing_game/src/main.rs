use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let x = rand::rng().random_range(1..=100);
    let y = rand::rng().random_range(1..=100);
    println!("x = {x}, y = {y}, x + y = {}", x + y);

    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the number!");

    println!("What game mode do you want?");

    let mut mode: u32;

    loop {
        println!("Enter 1 for human player, 2 for computer player.");

        let mut mode_input: String = String::new();
        let result = io::stdin().read_line(&mut mode_input);

        match result {
            Ok(_) => {}
            Err(_) => {
                println!("Please enter 1 or 2.");
                continue;
            }
        }

        mode = match mode_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };

        if mode >= 1 && mode <= 2 {
            break;
        }
        println!("Please enter 1 or 2.");
    }

    println!("Mode {}", mode);

    loop {
        let guess: u32;

        if mode == 1 {
            println!("Please input your guess.");

            let mut guess_input = String::new();
            io::stdin()
                .read_line(&mut guess_input)
                .expect("Failed to read line");

            guess = match guess_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number.");
                    continue;
                }
            };
        } else {
            guess = rand::rng().random_range(1..=100);
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
