#[derive(Debug)]
enum UsState {
    Florida,
    Georgia,
    Massachusetts,
    Minnesota,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Match statement
    println!("Hello, world!");
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter1 = Coin::Quarter(UsState::Georgia);
    let quarter2 = Coin::Quarter(UsState::Florida);
    let quarter3 = Coin::Quarter(UsState::Massachusetts);
    let quarter4 = Coin::Quarter(UsState::Minnesota);
    println!("The value of a penny is {} c", value_in_cents(penny));
    println!("The value of a nickel is {} c", value_in_cents(nickel));
    println!("The value of a dime is {} c", value_in_cents(dime));
    println!("The value of a quarter is {} c", value_in_cents(quarter1));
    println!("The value of a quarter is {} c", value_in_cents(quarter2));
    println!("The value of a quarter is {} c", value_in_cents(quarter3));
    println!("The value of a quarter is {} c", value_in_cents(quarter4));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}
