use std::io;

fn main() {
    println!("Get the nth fibonacci number!");
    println!("Enter a number");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline!");

    let input: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = fib(input);

    println!("{result}");
}

fn fib(n: u8) -> u8 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
