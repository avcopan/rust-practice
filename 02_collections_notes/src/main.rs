// // 01
// // The empty tuple `()` is called the "unit" -- used like `None` or `void
// // Note: You *can't* iterate over tuples!
// fn main() -> () {
//     let point: (i64, i64, i64) = (1, 2, 3);
//     let x = point.0;
//     let y = point.1;
//     let z = point.2;

//     let (x_, y_, z_) = point;
//     println!("{}, {}, {}", x, y, z);
//     println!("{}, {}, {}", x_, y_, z_);
// }

// // 02
// struct Point {
//     x: i64,
//     y: i64,
//     z: i64,
// }

// fn add_points(p1: Point, p2: Point) -> Point {
//     Point {
//         x: p1.x + p2.x,
//         y: p1.y + p2.y,
//         z: p1.z + p2.z,
//     }
// }

// fn main() -> () {
//     let p1: Point = Point { x: 1, y: 2, z: 3 };
//     let p2: Point = Point { x: 4, y: 5, z: 6 };
//     let mut p3: Point = add_points(p1, p2);

//     // mutating
//     p3.y = -2;

//     // destructuring
//     let Point {x, y, z} = p3;

//     // // destructuring if you want to ignore
//     // let Point {x, y: _, z} = p3;
//     // let Point {x, y, ..} = p3;

//     println!("p3: {}, {}, {}", x, y, z);
// }

// 03
fn main() -> () {
    // Note: All elements must have the same type
    // Also, the array has a hard-coded length
    let mut years: [i32; 3] = [1995, 2000, 2005];

    years[1] = 2003;

    let first_year = years[0];
    let [_, second_year, third_year] = years;

    println!("{}, {}, {}", first_year, second_year, third_year);

    for year in years.iter() {
        println!("Year: {}", year);
    }
}
