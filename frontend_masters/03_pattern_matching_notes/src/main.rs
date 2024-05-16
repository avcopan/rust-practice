enum Color {
    Red,
    Green,
    Blue,
    Custom {
        red: u8,
        green: u8,
        blue: u8,
    },
}

// Functions that are namespaced within the Color namespace
impl Color {
    // // Function namespace syntax
    // fn rgb(color: Color) -> (u8, u8, u8) {
    //     match color {
    //         Color::Red => { (255, 0, 0) }
    //         Color::Green => { (0, 255, 0) }
    //         Color::Blue => { (0, 0, 255) }
    //         Color::Custom { red, green, blue } => { (red, green, blue) }
    //     }
    // }
    // Method syntax
    fn rgb(self: &Color) -> (u8, u8, u8) {
        match self {
            Color::Red => { (255, 0, 0) }
            Color::Green => { (0, 255, 0) }
            Color::Blue => { (0, 0, 255) }
            Color::Custom { red, green, blue } => { (red, green, blue) }
        }
    }
}

fn color_string(color: Color) -> String {
    match color {
        Color::Red => { "red".to_string() }
        Color::Green => { "green".to_string() }
        Color::Blue => { "blue".to_string() }
        Color::Custom { red, green, blue } => { format!("{} {} {}", red, green, blue).to_string() }
        // // Alternatively, you could use a catch-all branch
        // _ => {
        //     "something else".to_string()
        // }
    }
}

fn main() {
    let go = Color::Green;
    let stop = Color::Red;
    let slow_down = Color::Custom {
        red: 255,
        green: 255,
        blue: 0,
    };

    let (r, g, b) = slow_down.rgb();

    println!("go: {}", color_string(go));
    println!("stop: {}", color_string(stop));
    println!("slow_down: {}", color_string(slow_down));
    println!("r g b: {} {} {}", r, g, b);
}
