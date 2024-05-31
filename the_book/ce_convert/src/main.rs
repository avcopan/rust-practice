use std::io;
use std::io::Write;

fn main() {
    // Print without newline, then flush to avoid line buffering
    print!("Temperature (F): "); 
    io::stdout().flush().unwrap();

    // Initialize a temperature variable and accept user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Parse the string to get a number
    let temp_f: f64 = match input.parse() {
        Ok(num) => num,
        Err(_) => -999.9,
    };

    // Early return for invalid input
    if temp_f < -459.67 {
        println!("{input} is not a valid temperature in Fahrenheit");
        println!("Please enter a number greater than -459.67");
        return;
    }

    let temp_c = ((temp_f - 32.0) * 5.0) / 9.0;
    println!("Temperature (C): {temp_c}");
}
