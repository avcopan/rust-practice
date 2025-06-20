fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        -1
    };

    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    println!("gopher: {}", animal_habitat("gopher"));
    println!("snake: {}", animal_habitat("snake"));
    println!("crab: {}", animal_habitat("crab"));
    println!("lion: {}", animal_habitat("lion"));
}
