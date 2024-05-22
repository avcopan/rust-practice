use rand::Rng;
use std::cmp::Ordering;
use std::io; // Rng is a "trait", defining the functionality of a type

fn main() {
    println!("I have a number in mind. Can you guess it?");

    // rand::thread_rng returns a random number generator on the current thread
    // gen_range generates random numbers in the range start..=100 (inclusive!!)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();
        // v| io:: calls the io library imported above
        io::stdin()
            // v| &mut passes the variable as a mutable reference:
            //  - &: passing by reference avoids creating a copy in memory
            //  - mut: by default, references are immutable, so we need to declare
            //  them as mutable if we want to allow the function to change our
            //  variable
            .read_line(&mut guess)
            // v| the next line operates on the return value of `read_line()`, which
            // is a `Result` enum: (Result val).expect()
            //  - Result enums have two possible values: Result::Ok or Result::Err
            //  - If the result is Ok, expect() returns the value that Ok is holding
            //  - If the result is Err, expect() crashes the program with the error message we specify
            .expect("Failed to read line");

        // v| "shadowing" allows us to reuse the `guess` variable name while converting its type
        // v| the parse method parses the string into another type -- in this case,
        // the requested guess type
        // v| since the parsing can fail, it returns another `Result` enum
        //  - instead of doing a .expect() and failing, here we handle the error
        //  ang allow the program to continue when the guess fails
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please guess a number!");
                continue;
            },
        };

        println!("You guessed: {guess}");

        // v| `cmp` is method that can be called on anything that can be compared
        //  - in this case, it is being called on the variable `guess`
        //  - it returns an `Ordering` enum with variants `Less`, `Greater`, `Equal`
        // v| the `match` expression allows us to decide how to proceed based on the
        // return value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

// /* NOTES: */
// fn main() {
//     // Immutable variable (default)
//     let apples = 5;
//     println!("Apples: {apples}");
//     // Mutable variable
//     let mut bananas = 6;
//     println!("Bananas: {bananas}");
//     bananas = 7;
//     println!("Bananas: {bananas}");
// }
