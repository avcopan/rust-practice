fn bigger(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn main() {
    println!("10 ? 8: {}", bigger(10, 8));
    println!("32 ? 42: {}", bigger(32, 42));
    println!("32 ? 32: {}", bigger(32, 32));
}
