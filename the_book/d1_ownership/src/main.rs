// Rust ownership rules:
//  1. Each value has an *owner*
//  2. There can only be one owner at a time
//  3. When the owner goes out of scope, the value is dropped
fn main() {
    // Variable Scope
    {
        let s = "hello"; // `s` comes into scope
        // here, we are using a string literal, which are values hardcoded into
        // the program; unlike `String` values, string literals can be stored on
        // the stack; string literals are *always* immutable!
        println!("{s} there"); // `s` is in scope, so you can do stuff with it
    } // `s` goes out of scope

    // The `String` type
    //  - unlike str literals, Strings can be made mutable with the mut keyword
    //  - allowing them to be mutable means they must be stored on the *heap*,
    //  because the necessary memory allocation is not known at compile time
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literal to the `String`
    println!("{}", s);

    // Memory and Allocation
    //  - as an immutable value, a string literal is known at compile time and
    //  hardcoded directly into the executable
    //  - a `String`, on the other hand, must implement a mutable, growable
    //  piece of text, whose size is unknown at compile time
    //      - it therefore requires allocation of appropriate amounts of memory
    //      on the heap *at runtime*
    //      => this means we need a way of returning the memory to the allocator
    //      when we're done using it!
    //  - different ways of freeing memory:
    //      1. garbage collector - keeps track and frees unused memory
    //      2. explicit freeing - manually free the memory in the code
}
