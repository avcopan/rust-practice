#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> u32 {
        println!("Calling width as a method!!");
        self.width
    }
}

// Impl blocks can be separated as needed
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated functions
    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    // Method Syntax
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };

    println!("{:#?}", rect1);

    // Defining Methods
    println!("Area: {}", rect1.area());

    //  - Method names can overlap with field names
    println!("The width field is {}", rect1.width);
    //  - Note that `rect1.width()` is equivalent to `(&rect1).width()`
    //  - Rust automatically infers how `self` needs to be received by the
    //  function
    println!("The width method returns {}", rect1.width());

    // Methods with More Parameters
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions
    let square = Rectangle::create_square(10);
    println!("{:#?}", square);
}
