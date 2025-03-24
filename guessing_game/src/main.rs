use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_mode() -> u32 {
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
    return mode;
}

fn get_difficulty() -> u32 {
    println!("Please choose a difficulty.");

    let mut difficulty: u32;

    loop {
        println!("Easy (1), Medium (2), or Hard (3)?");

        let mut difficulty_input: String = String::new();
        if let Err(_) = io::stdin().read_line(&mut difficulty_input) {
            println!("Please enter 1, 2 or 3.");
            continue;
        }

        difficulty = match difficulty_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };

        if difficulty >= 1 && difficulty <= 3 {
            break;
        }
        println!("Please enter 1, 2 or 3.");
    }
    return difficulty;
}

fn human_game(max: u32) {
    let secret_number = rand::rng().random_range(1..=max);

    loop {
        println!("Please input your guess - between 1 and {}", max);

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };

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

fn computer_game(max: u32) {
    let secret_number = rand::rng().random_range(1..=max);

    let mut guess: u32 = rand::rng().random_range(1..=100);
    loop {
        println!("Computer guessed: {}", guess);

        let guess_direction: i32;
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_direction = 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_direction = -1
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // Create a new guess based on direction
        if guess_direction < 0 {
            guess = rand::rng().random_range(1..=(guess - 1));
        } else {
            guess = rand::rng().random_range((guess + 1)..=max);
        }
    }
}

fn main() {
    println!("Guess the number game!");

    let mode = get_mode();
    let difficulty = get_difficulty();

    let max: u32;

    match difficulty {
        1 => max = 10,
        2 => max = 100,
        3 => max = 1000,
        _ => {
            println!("Unexpected difficulty!");
            return;
        }
    }

    if mode == 1 {
        println!("Human game - go!");
        human_game(max);
    } else {
        println!("Computer game - go!");
        computer_game(max);
    }
}
