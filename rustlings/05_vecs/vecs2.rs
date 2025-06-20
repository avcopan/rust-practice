fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(element * 2);
    }

    output
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| element.pow(2)).collect()
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("vec_loop: {:?}", vec_loop(&a[2..5]));
    println!("vec_map: {:?}", vec_map(&a[2..5]));
}