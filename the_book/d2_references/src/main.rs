fn main() {
    // References and Borrowing
    //  - references allow you to refer to a value *without* taking ownership
    //  - a reference is "an address we can follow to access data stored at that
    //  address", similar to a pointer, except "a reference is guaranteed to
    //  point to a valid value of a particular type for the life of that
    //  reference"
    let s_in = String::from("hello");
    let len = calculate_length(&s_in);
    // the reference *refers* to the value of `s_in` but does not own it
    println!("The length of '{}' is {}", s_in, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` is not dropped when it goes out of scope, because it is a reference
