use std::convert::TryInto;

fn call_me(num: u8) {
    for i in 0..num {
        println!("[Ring] Call number {}", i + 1);
    }
}

fn main() {
    let two_u8: u8 = 2;
    let two_i32: i32 = 2;
    call_me(two_u8.pow(3));
    call_me(two_i32.pow(3).try_into().unwrap());
}
