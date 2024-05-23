const HOURS_TO_SECONDS: i32 = 60 * 60;

fn main() {
    // Mutable vs immutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;  // would raise an error without `mut` keyword
    println!("The value of x is: {x}");

    // Constants (declared above)
    //  - cannot be designated as mutable
    //  - type must be annotated
    //  - *must be set to a constant expression, not computed at runtime*
    println!("{x} hours is {} seconds", x * HOURS_TO_SECONDS);

    // Shadowing
    //  - Indicated by repeated use of the `let` keyword
    //  - A new variable takes over the name of a previous variable
    //  - Difference from making variable mutable with `mut`:
    //      - Once set, the variable immutable
    //      - We can change types
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y in the outer scope is still {y}");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces.")
}
