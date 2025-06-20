fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        "Don't like it."
    }
}

fn main() {
    println!("strawberry: {}", picky_eater("strawberry"));
    println!("potato: {}", picky_eater("potato"));
}
