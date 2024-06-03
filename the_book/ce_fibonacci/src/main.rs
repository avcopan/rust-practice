use std::io;
use std::io::Write;

fn main() {
    // Request user input
    println!("Calculate the nth Fibonacci number...");
    print!("n = ");
    io::stdout().flush().unwrap();

    // Accept user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Parse the string to get a number
    let num: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    // Early return for invalid input
    if num <= 0 {
        println!("{input} is not a valid index in the Fibonacci sequence.");
        println!("Please enter a natural number.");
    }

    // Calculate the fibonacci number
    let fib = fibonacci(num);
    println!("F(n) = {fib}");
}

fn fibonacci(num: u32) -> u128 {
    let mut fib0: u128 = 0;
    let mut fib1: u128 = 1;
    let mut fib2: u128 = 1;
    for _ in 1..num {
        fib2 = fib0 + fib1;
        fib0 = fib1;
        fib1 = fib2;
    }
    fib2
}
