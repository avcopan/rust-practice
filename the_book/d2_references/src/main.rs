fn main() {
    // References and Borrowing
    //  - references allow you to refer to a value *without* taking ownership
    //  - a reference is "an address we can follow to access data stored at that
    //  address", similar to a pointer, except "a reference is guaranteed to
    //  point to a valid value of a particular type for the life of that
    //  reference"
    //  - the reference acts as a pointer *to the pointer*, which allows it to
    //  refer to the value without owning it
    let s_in = String::from("hello");
    //  - the act of creating a reference is called *borrowing*
    //  - the function is allowed to *borrow* the value, but it does not own it
    //  (it has not been *moved*)
    let len = calculate_length(&s_in);
    // the reference *refers* to the value of `s_in` but does not own it
    println!("The length of '{}' is {}", s_in, len);

    // Mutable References
    //  - allow the borrower to change the value
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    //  - restriction: we cannot borrow a reference as mutable more than once at
    //  a time
    //  - the benefit of this restriction is that it prevents *data races*
    //      - when two pointers access the same data simultaneously and (at
    //      least) one of them is mutating it, the resulting behavior is
    //      undefined -- the value of each variable depends on which one gets
    //      there first
    //  - Rust prevents the conditions leading to such data races from the
    //  outset -- code that can cause data races won't compile
    let mut s0 = String::from("hello");
    let s1 = &mut s0; // first mutable borrow
    // let s2 = &mut s0; // second mutable borrow
    // ^ Not allowed here, because creating a duplicate mutable reference
    println!("{}", s1);
    let s2 = &mut s0;
    // ^ OK here, because `s1` is no longer being used
    //  - I think this is because, in practice, the variable gets freed the last
    //  time it is used before going out of scope
    println!("{}", s2);

    //  - one can create immutable references from mutable variables
    let mut s0 = String::from("hello");
    s0.push_str(" my immutable friend"); // no problem
    //  - in fact, one can create as many immutable references as one likes
    let s1 = &s0; // no problem
    let s2 = &s0; // no problem
    //  - HOWEVER, they cannot be mutated or borrowed as mutable while the
    //  immutable reference is scope
    // s0.push_str(" my immutable friend"); // BIG PROBLEM
    // ^ cannot borrow `s0` as mutable because it is also borrowed as immutable
    // let s3 = &mut s0; // BIG PROBLEM
    // ^ cannot borrow `s0` as mutable because it is also borrowed as immutable
    println!("{}", s1);
    println!("{}", s2);
    // println!("{}", s3);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("world!");
    // ^ error: cannot borrow `*s` as mutable, as it is behind a `&` reference
    s.len()
} // `s` is not dropped when it goes out of scope, because it is a reference

fn change(s: &mut String) {
    s.push_str(", world");
}
