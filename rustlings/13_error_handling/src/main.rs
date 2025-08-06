use std::num::ParseIntError;
use std::cmp::Ordering;

// Exercise 1
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

// Exercises 2-3
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // The "long" way
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // Shorter way that is equivalent: use `?` to propagate the error
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// Exercise 4
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // // Initial solution:
        // if value == 0 {
        //     return Err(CreationError::Zero);
        // }
        // if value < 0 {
        //     return Err(CreationError::Negative);
        // }
        // Ok(Self(value as u64))
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    let x = PositiveNonzeroInteger::new(10);
    println!("x = {:?}", x);

    let x = PositiveNonzeroInteger::new(-10);
    println!("x = {:?}", x);
}

// // Exercise 3:
// fn main() -> Result<(), ParseIntError> { // Note: You have to change the "main" function signature
//     let mut tokens = 100;
//     let pretend_user_input = "8";
//     // let pretend_user_input = "80";
//     // let pretend_user_input = "beep boop";

//     let cost = total_cost(pretend_user_input)?;

//     if cost > tokens {
//         println!("You can't afford that many!");
//     } else {
//         tokens -= cost;
//         println!("You have {tokens} tokens.");
//     }

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    // Exercise 1
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }

    // Exercise 2
    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }

    // Exercise 4
    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
