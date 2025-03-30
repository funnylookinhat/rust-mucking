use std::env;

fn print_usage() {
    println!("Print some powers of numbers! Exciting!");
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
        let arg_number: i64 = match args[i].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a number - skipping", args[i]);
                continue;
            }
        };

        let powers = get_powers(arg_number);

        println!("Powers of {arg_number}");
        for (j, &power) in powers.iter().enumerate() {
            let str_result = power.map_or_else(|| "Overflow!".to_string(), |n| format!("{:#?}", n));
            println!("    {arg_number}^{} = {}", j + 1, str_result);
        }
    }
}

fn get_powers(x: i64) -> [Option<i64>; 4] {
    [
        x.checked_pow(1),
        x.checked_pow(2),
        x.checked_pow(3),
        x.checked_pow(4),
    ]
}
