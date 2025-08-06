fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day < 22 {
        return Some(5);
    } else if hour_of_day < 24 {
        return Some(0);
    } {
        return None;
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // In pattern matching, the `ref` keyword creates a reference to a value
    // within the pattern, instead of moving it or copying the value.
    // Without this, the value would move to `p` and the `println` line below
    // would raise a compiler error.
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1
    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12).unwrap();

        assert_eq!(ice_creams, 5); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }

    // Exercise 2
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    // TODO: Finish this!
    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // Syntax:
        //     while let PATTERN = EXPRESSION {
        //         // Code to execute if the pattern matches
        //     }
        // It loops until the expression no longer matches the pattern.
        // In the following case, the pattern is a nested `Some(Some(i))`.
        while let Some(Some(integer)) = optional_integers.pop() {
            println!("integer = {:?}", integer);
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
