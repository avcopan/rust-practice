// // 1
// fn main() {
//     let name = "Andreas";
//     println!("Hello, {}!", name);
// }

// // 2
// fn main() {
//     let x: f64 = 1.1;  // explicit type annotation
//     let y = 2.2;  // type inference
//     // y = "A string";  // this doesn' work (even if mutable), because type changes

//     println!("x * y = {}", x * y);
// }

// // 3
// fn multiply(x: i64, y: u8) -> i64 {
//     return x * (y as i64);
// }

// fn main() {
//     let negative_ninety: i64 = -90;
//     let two: u8 = 2;

//     println!("result: {}", multiply(negative_ninety, two));
// }

// // 4
// fn main() {
//     let cats = 3;

//     if cats > 1 {
//         println!("multiple cats!")
//     } else {
//         println!("one or fewer cats")
//     }
// }

// // 5
// // fn multiply(x: f64, y: f64) -> f64 {
// //     return x * y;  // statement
// // }

// fn multiply(x: f64, y: f64) -> f64 {
//     x * y  // expression with explicit return
// }

// fn main() {
//     println!("result: {}", multiply(4., 7.));
// }

// 6
fn main() {
    let cats = 1;
    let message = if cats > 1 {
        "Multiple cats!"
    } else {
        "Need more cats!"
    };
    println!("message: {}", message);
}
