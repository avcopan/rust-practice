// #[derive(Debug)]
// enum IpAddress {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

fn main() {
    // let home = IpAddress::V4(127, 0, 0, 1);
    // let loopback = IpAddress::V6(String::from("::1"));

    // println!("{:#?}", home);
    // println!("{:#?}", loopback);

    // The Option enum
    //  - The Option enum prevents us from using Option<T> as if it were a T
    //  value; It forces us to deal with the possibility of an absent value
    //  - This also means that when we have T, we can proceed confidently
    //  knowing that the value is valid -- if not, the compiler will tell us
    let some_char = Some('e');
    let some_number = Some(5);
    // let sum = some_number + 5;
    // ^ error: cannot add `{integer}` to `Option<{integer}>`
    let absent_number: Option<i32> = None;
    println!("{:?}", some_char);
    println!("{:?}", some_number);
    println!("{:?}", absent_number);
}
