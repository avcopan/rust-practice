#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of this rectangle is {}", area(&rect1));
    // The following only works since we passed by reference -- otherwise, the
    // `rect1` variable would have moved into the `area` function and be freed
    // at when it goes out of scope there
    println!("{:?}", rect1);
    //  - Alternative print, a little nicer (multiline)
    println!("{:#?}", rect1);
    //  - The debugging macro prints the line number and can be used on
    //  expressions as well as individual variables
    dbg!(rect1.width * rect1.height);
    //  ^ probably makes more sense inside the area function below
    //  - *Note* however that it does take ownership of our variable, we pass a
    //  reference if we don't want to lose it
    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    dbg!(rect.width * rect.height);
    rect.width * rect.height
}
