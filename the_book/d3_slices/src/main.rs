
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

    // String Slices as Parameters
    let s = "The quick brown fox jumps over the lazy dog.";
    println!("{}", first_word(s));
    //  ^ string slices are literals already, so no need for a `&`
    println!("{}", first_word(&s[4..]));
    println!("{}", first_word(&s[10..12]));
    println!("{}", first_word(&s[35..]));

    // Array Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice);
}

// &str is the "string slice" type
// fn first_word(s: &String) -> &str {
//  - by changing the argument to `&str` as well, we allow the function to
//  handle *either* slices or whole string references
fn first_word(s: &str) -> &str {
    let b = s.as_bytes();

    for (index, &bchar) in b.iter().enumerate() {
        if bchar == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
