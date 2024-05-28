fn main() {
    // if expressions
    //  - in Rust, like in Python, `if` is an *expression*
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {:?}", number);
    let nothing = if condition {} else {};
    println!("The value of nothing is: {:?}", nothing);
    //  - the result types of the expression must be compatible
    //  - reason: Rust needs to know the types of variables *at compile time*
    // // this will not compile:
    // let thing = if condition { 5 } else { "six" };
    // println!("The value of thing is: {:?}", thing);

    // loops
    //  - `loop` repeats indefinitely unless terminated by an explicit break statement
    let mut i = 0;
    loop {
        if i == 2 {
            i += 1;
            continue;
        }
        if i > 5 {
            break;
        }
        println!("again! {i}");
        i += 1;
    }
    //  - loops can be used to retry an operation you know might fail, "such as
    //  checking whether a thread has completed its job"
    //  - the break expression allows you to pass a value out of the loop, e.g.
    let mut counter = 2;

    let result = loop {
        counter *= counter;
        if counter > 100 {
            break counter;
        }
    }; // note: we need a semicolon to end the statement

    // loop labels (very cool!!)
    // (left off here)

    println!("The result is {result}");
}
