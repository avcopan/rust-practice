#[derive(Debug)] // allows the struct to be printed
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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
}
