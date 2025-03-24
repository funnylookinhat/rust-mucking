fn main() {
    let mut x = (1, 2, 3);

    println!("x 1 {} 2 {} 3 {}", x.0, x.1, x.2);

    x.0 = x.0 * 3;
    x.1 = x.1 * 3;
    x.2 = x.2 * 3;

    println!("x 1 {} 2 {} 3 {}", x.0, x.1, x.2);

    let status = ('🤡', 1);

    let (c, d) = status;

    println!("Status: {} {}", c, d);
}
