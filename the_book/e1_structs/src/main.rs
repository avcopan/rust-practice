#[derive(Debug)] // allows the struct to be printed
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)] // allows the struct to be printed
struct Color(i32, i32, i32);
#[derive(Debug)] // allows the struct to be printed
struct Point(i32, i32, i32);

#[derive(Debug)] // allows the struct to be printed
struct DatalessType;


fn new_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand for username: username
        email, // shorthand for email: email
        sign_in_count: 1,
    }
}

fn main() {
    // Defining and Instantiating Structs
    let user1 = User {
        active: true,
        username: String::from("bobhuffligan"),
        email: String::from("bobhuffligan@gmail.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    println!("{:?}", user1);

    let mut user2 = new_user(
        String::from("jimshmilligan@gmail.com"),
        String::from("jimshmilligan")
    );
    user2.active = false;
    user2.email = String::from("jimshmilligan@yahoo.com");
    user2.sign_in_count += 1;
    println!("{:?}", user2);

    // Creating Instances from Other Instances with Struct Update Syntax
    //  - instantiating with `..<other instance>` fills in missing values using
    //  another instance
    let user3 = User {
        email: String::from("bobhyffligan@yahoo.com"),
        ..user1
    };
    println!("{:?}", user3);
    //  - NOTE: This **moves** any heap variables from `user1` over to `user3`
    //  - In this case, user1.username is moved to user3.username
    // println!("{:?}", user1);
    //  ^ error: borrow of partially moved value: `user1`

    //  - The stack variables get copied, so, if we replace all of the heap
    //  variables, then the original instance that we update from remains valid
    let user4 = User {
        username: String::from("jimshmil"),
        email: String::from("jimshmil@hotmail.com"),
        ..user2
    };
    println!("{:?}", user4);
    println!("{:?}", user2);

    // Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let point2 = Point(-5, 8, -9);
    println!("{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
    println!("{}", point2.0);
    println!("{}", point2.1);
    println!("{}", point2.2);
    println!("{:?}", black);
    println!("{:?}", origin);
    println!("{:?}", point2);

    // Unit-Like Structs Without Any Fields
    let d = DatalessType;
    println!("{:?}", d);
}
