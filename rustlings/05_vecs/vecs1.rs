fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array
    let v = a.to_vec();

    (a, v)
}

fn main() {
    let (a, v) = array_and_vec();
    println!("a: {:?}", a);
    println!("v: {:?}", v);
}
