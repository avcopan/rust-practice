
fn main() {
    // String Slices
    //  - the slice data structure stores a pointer to the starting position and
    //  the length of the lice
    let mut s = String::from("hello world");
    let hello = &s[0..5]; // equivalent to &s[..5]
    let world = &s[6..11]; // equivalent to &s[6..]
    println!("{}", hello);
    println!("{}", world);
    let word = first_word(&s);
    println!("first word: {}", word);
    s.clear(); // resets the string
    //  ^ clear requires a mutable reference, which only works if there are no
    //  other references in scope
    //  - if we keep any of the slices in scope by using them after this point,
    //  then the above will throw a compiler error
    //  - for example:
    // println!("first word: {}", word);
    //  ^ error: cannot borrow `s` as mutable because it is also borrowed as
    //  immutable

    // String Literals as Slices
    //  - surprisingly, string literals have the "string slice" type, `&str`
    //  - they are implemented as slices pointing to a specific point in the
    //  binary; this is also why they are immutable!
    let s = "Hello, world!";
    println!("{}", s);
}

// &str is the "string slice" type
fn first_word(s: &String) -> &str {
    let b = s.as_bytes();

    for (index, &bchar) in b.iter().enumerate() {
        if bchar == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
