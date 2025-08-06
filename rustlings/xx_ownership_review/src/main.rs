fn main() {
    let x = String::from("hello");
    let y = x; // Data *moved* from x to y -- x no longer valid!
    // println!("x = {x}, y = {y}");
    println!("y = {y}");
}
