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
    //      3. a different path...
    //  - Rust frees memory automatically once the variable that *owns* it goes
    //  out of scope
    {
        let mut s = String::from("hi again");
        // String::from allocates memory on the heap to store the value
        // the variable `s` "owns" the allocated memory
        s.push_str(", you.");
        println!("{}", s);
    } // as the variable `s` goes out of scope, the memory is freed

    // Variables and Data Interacting with Move
    //  - with stack variables, the following code sets both `x` and `y` to the
    //  value `5` -- `y` *copies* the value of `x`, and each is pushed onto the
    //  stack as a separate variable
    let x = 5;
    let y = x;
    println!("x = {x}");
    println!("y = {y}");
    //  - with heap variables, `s1` stores a *pointer* and `s2` copies the
    //  *pointer value*, so that they now both point to the same allocated
    //  memory on the heap
    //  - to ensure memory safety, after the line `let s2 = s1`, Rust considers
    //  `s1` as **no longer valid** -- the ownership **moves** from `s1` to `s2`
    let s1 = String::from("hello");
    let s2 = s1; // `s1` is **moved** to `s2`
    // println!("s1 = {s1}");  // compiler error: borrow of a moved value: `s1`
    println!("s2 = {s2}");
    //  - now `s1` is invalid and `s2` owns the memory
    //  - when `s2` goes out of scope, the allocated memory on the heap is freed

    // Variables and Data Interacting with Clone
    //  - if we want to copy, we can explicitly clone the variable to allocate a
    //  new, independent block of memory on the heap storing the same value
    let t1 = String::from("HELLO");
    let t2 = t1.clone();
    //  - clone acts as a visual indicator that something is going on, and the
    //  operation might be expensive
    //  - if you don't need to call clone, you can rest assured that the
    //  operation should be inexpensive
    println!("t1 = {t1}");
    println!("t2 = {t2}");

    // Note: Stack variables *are* automatically copied, because they have a
    // known size at compile time and the copy operation is trivial
    //  - when implementing our own types, any type that has a known size at
    //  compile time can be given the `Copy` trait to be automatically copied
    //  - Stack / automatically copying types:
    //      1. Booleans and numbers (bool, u32, i32, f64)
    //      2. Characters (char)
    //      3. Tuples containing only the above types, e.g. ((char, char, bool))
    //  - If a Tuple contains any non-stack type, it must be stored on the heap
    //  and allocated (?)

    // Ownership and Functions
    //  - passing *into* a function:
    //      - for heap variables, the value *moves* into the function when it is
    //      called and is no longer valid in the calling scope
    //      - for stack variables, the value is copied and can still be called
    let s = String::from("hello again");
    takes_ownership(s);
    // println!("{}", s);  // `s` has moved, so it is no longer valid here

    //  - returning *from* a function:
    //      - for heap variables, the value moves out of the function and
    //      becomes valid in the calling scope
    //      - for stack variables, the value gets copied
    let s = gives_ownership();
    println!("{}", s);

    //  - passing into and returning from a function:
    //      - the value of heap variables can preserved in scope by returning
    //      them back from the function
    let s_in = String::from("try this one!");
    let s_out = takes_and_gives_back(s_in);
    println!("{}", s_out);
    //      - we can return them as tuples
    let (s_out, len) = calculate_length(s_out);
    println!("'{}' has length {}", s_out, len);
    //      - but the better way is to pass by reference...
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("here you go!");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
