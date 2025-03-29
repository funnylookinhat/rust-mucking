use std::env;

fn print_usage() {
    println!("Usage: rust4 x [y] [z] ...");
    println!("You may provide as many numbers as you want.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please provide at least one number.");
        print_usage();
        std::process::exit(1);
    }

    if args[1] == "--help" || args[1] == "-h" {
        print_usage();
        std::process::exit(0);
    }

    for i in 1..args.len() {
        let arg_number: u32 = match args[i].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number - skipping", args[i]);
                continue;
            }
        };

        let powers = get_powers(arg_number);

        println!("Powers of {arg_number}");
        for j in 0..powers.len() {
            println!(
                "    {arg_number}^{} = {}",
                j + 2,
                if powers[j] == 0 {
                    format!("Overflow!")
                } else {
                    format!("{}", powers[j])
                }
            );
        }
    }
}

fn get_powers(x: u32) -> [u32; 3] {
    [
        x.checked_pow(2).unwrap_or(0),
        x.checked_pow(3).unwrap_or(0),
        x.checked_pow(4).unwrap_or(0),
    ]
}
