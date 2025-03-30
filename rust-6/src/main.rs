use clap::{Arg, ArgAction, Command, arg};

fn main() {
    let matches = Command::new("ArgParserTest")
        .version("1.33.7")
        .about("Testing command parsing with clap...")
        .arg(Arg::new("foo"))
        .arg(Arg::new("bar"))
        .arg(Arg::new("baz"))
        .arg(Arg::new("qux").action(ArgAction::Append))
        .arg(arg!(--first <VALUE>).required(true))
        .arg(arg!(--second <VALUE>).required(true))
        .get_matches();

    println!(
        "one: {:?}",
        matches.get_one::<String>("first").expect("required")
    );
    println!(
        "two: {:?}",
        matches.get_one::<String>("second").expect("required")
    );

    println!(
        "foo: {:?}",
        matches
            .get_one::<String>("foo")
            .map_or_else(|| "missing".to_string(), |s| format!("{}", s))
    );

    println!(
        "bar: {:?}",
        matches
            .get_one::<String>("bar")
            .map_or_else(|| "missing".to_string(), |s| format!("{}", s))
    );

    println!(
        "baz: {:?}",
        matches
            .get_one::<String>("baz")
            .map_or_else(|| "missing".to_string(), |s| format!("{}", s))
    );

    let qux = matches
        .get_many::<String>("qux")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    for (_, &v) in qux.iter().enumerate() {
        println!("Qux Value: {}", v);
    }
}
