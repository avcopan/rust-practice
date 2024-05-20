use std::io;

fn main() {
    println!("I have a number in mind. Can you guess it?");
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

    println!("You guessed: {guess}");
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
