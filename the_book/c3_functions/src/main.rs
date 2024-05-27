fn main() {
    another_function(5);
    yet_another_function();

    println!("five() returns {}", five());
    println!("plus_one(7) returns {}", plus_one(7));
}

// Functions
//  - By design, type annotations are *required* in the function signature
//  - The benefit is that...
//      a. The compiler (/VSCode extension) can readily identify what can and
//      can't be done with a given argument and give more helpful error messages
//      b. This becomes almost the *only* place that type annotations are
//      required -- they can be inferred almost everywhere else
fn another_function(x: i32) {
    println!("The value of x is {x}")
}

// Statements and expressions
//  - Function bodies are made up of a series of a *statements* optionally
//  ending in an *expression*...
//      fn {
//          statement 1; // Statements are ended by semicolons
//          statement 2;
//          statement 3;
//          ...
//          statement n;
//          expression // No semicolon! That would turn it into a statement!
//      }
//  - Unlike statements, expressions evaluate to a resultant value
//  - Rust is an expression-based language, so this is an important definition
//  to understand
fn yet_another_function() {
    // Statements do not return values...
    //  - You can't assign the result to another variable, e.g.
    //      let x = (let y = 6); // Does not work!
    //  - This is unlike Python, where you can do x = y = 6
    let y = 6;
    println!("The value of y is {y}");

    // Expressions evaluate to some value, which is returned by the expression...
    //  - Ex. 5 + 6 (=> 11)
    //  - A new scope block created with curly brackets is an expression, e.g.
    let y = {
        // The curly braces enclose an expression (that contains a statement)
        let x = 3;
        x + 1 // Note that we *don't* put a semicolon here! Semicolons end statements!
    }; // (=> 4)

    println!("The value of y is {y}");
}

// Functions with return values
//  - You can return early using the return keyword, but most functions use
//  implicit return
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
