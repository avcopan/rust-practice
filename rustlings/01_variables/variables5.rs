fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a number: {number}");

    let number = 3; // shadowing is different from mutability
    println!("Number plus two is: {}", number + 2);
}