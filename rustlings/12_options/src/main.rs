fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day < 22 {
        return Some(5);
    } else if hour_of_day < 24 {
        return Some(0);
    } {
        return None;
    }
}


fn main() {
    // Left off learning while-let (see TODO below)
    // let mut optional = Some(0);

    // loop {
    //     match optional {
    //         Some(i) => {
    //             if i > 9 {
    //                 println!("Greater than 9, quit!")
    //             }
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1
    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
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
    // #[test]
    // fn layered_option() {
    //     let range = 10;
    //     let mut optional_integers: Vec<Option<i8>> = vec![None];

    //     for i in 1..=range {
    //         optional_integers.push(Some(i));
    //     }

    //     let mut cursor = range;

    //     // TODO: Make this a while-let statement. Remember that `Vec::pop()`
    //     // adds another layer of `Option`. You can do nested pattern matching
    //     // in if-let and while-let statements.
    //     integer = optional_integers.pop() {
    //         assert_eq!(integer, cursor);
    //         cursor -= 1;
    //     }

    //     assert_eq!(cursor, 0);
    // }
}
