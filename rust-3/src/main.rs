use clap::{Arg, command};

use rand::Rng;

fn main() {
    let chars = [
        "😀", "😂", "🥲", "😎", "🤩", "😡", "😭", "😴", "🥳", "🤯", "🤔", "🤖", "👻", "💀", "🎃",
        "😺", "🙀", "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷",
        "🐸", "🐵", "🦄", "🐔", "🐧", "🐦", "🐤", "🐢", "🐍", "🦖", "🐙", "🦑", "🦞", "🦐", "🦀",
        "🐡", "🐠", "🐬", "🦆", "🦉",
    ];

    let matches = command!()
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Number of lines to print")
                .value_parser(u32_parser)
                .default_value("10"),
        )
        .arg(
            Arg::new("emojis")
                .short('e')
                .long("emojis")
                .help("Number of emojis per line to print")
                .value_parser(u32_parser)
                .default_value("10"),
        )
        .get_matches();

    let lines: u32 = matches.get_one::<u32>("lines").copied().unwrap_or_else(|| {
        eprintln!("Please provide a number for lines.");
        std::process::exit(1);
    });

    let emojis: u32 = matches
        .get_one::<u32>("emojis")
        .copied()
        .unwrap_or_else(|| {
            eprintln!("Please provide a number for emojis.");
            std::process::exit(1);
        });

    for _i in 0..=lines {
        let mut s = String::new();
        for _j in 0..=emojis {
            let emoji = chars[rand::rng().random_range(1..=(chars.len() - 1))];
            s = s + emoji;
        }
        println!("{}", s);
    }
}

fn u32_parser(s: &str) -> Result<u32, String> {
    match s.trim().parse() {
        Ok(num) => Ok(num),
        Err(err) => Err(err.to_string()),
    }
}
