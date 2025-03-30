use std::env;

fn print_usage() {
    println!("Usage: rust5 n");
    println!("The value for n must be an integer.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please provide a number.");
        print_usage();
        std::process::exit(1);
    }

    let arg_number: i64 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{} is not a number.", args[1]);
            print_usage();
            std::process::exit(1);
        }
    };

    println!(
        "The number is {}.  We will find the power of the number which causes an overflow.",
        arg_number
    );

    let mut exponent = 1;
    let max_value = loop {
        let result = arg_number.checked_pow(exponent);
        match result {
            None => {
                break arg_number.checked_pow(exponent - 1).unwrap();
            }
            _ => {
                println!("{} ^ {} = {}", arg_number, exponent, result.unwrap())
            }
        };
        exponent += 1;
    };

    println!("The max exponential value is {max_value}");
}
