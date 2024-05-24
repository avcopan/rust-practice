use std::io;

fn main() {
    // Data type
    //  - rust is statically typed -- all types must be known at compile time
    //  - if the type can be inferred, type annotation is optional
    //  - otherwise, type annotation is required
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {guess}");

    // Scalar types
    // 1. Integers
    //      - Range for i(n): [-2^(n-1), 2^(n-1) - 1]; i8: [-128, 127]
    //      - Range for u(n): [0, 2^n - 1]; u8: [0, 255]
    let x = 244u8; // can add `u8` to the literal to specify type
    println!("x = {x}");
    // 2. Floating point numbers
    // 3. Booleans
    // 4. Characters
    let y = 'y';
    let z = '😻';
    println!("y = {y}");
    println!("z = {z}");

    // Compound types
    // 1. Tuples
    let t: (i32, f64, u8) = (500, 6.4, 1);
    println!("t = {:?}", t);
    println!("t.0 = {}, t.1 = {}, t.2 = {}", t.0, t.1, t.2);
    let (a, b, c) = t; // destructuring
    println!("a = {a}, b = {b}, c = {c}");
    let unit = (); // unit value is like a `None` (I think)
    println!("unit = {:?}", unit);
    // 2. Arrays
    //  - unlike a tuple, all elements must have the same type
    //  - have a *fixed length* in Rust (the standard library provides a
    //  `vector` type that can have variable size)
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation: [type; length]
    println!("a = {:?}", a);
    println!("a[2] = {:?}", a[2]);
    let b = [3; 5]; // 5 elements, all with value 3
    println!("b = {:?}", b);

    // Unlike other elements, Rust makes it impossible to overrun the bounds of
    // an array when accessing it.
    //  - If possible, it throws an error at compile time:
    // println!("a[5] = {:?}", a[5]); // throws index out of bounds error
    //  - Otherwise, it throws an error at runtime *before* running the
    //  problematic code, so that it always stays within the bounds of the array
    //      - Ex: If the user inputs 9 for the index below, the thread panicks
    //      with a runtime error *before* executing `a[index]`
    let mut index = String::new();
    println!("Enter an array index:");
    io::stdin().read_line(&mut index).expect("Failed to read input!");
    let index: usize = index.trim().parse().expect("Not a number!");
    let accessed_value = a[index];
    println!("accessed_value = {accessed_value}");
}
