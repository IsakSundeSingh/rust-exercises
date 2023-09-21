#![allow(unused_variables)]

/// Try to parse the input (which we only need to read, hence the reference)
/// as a number (an i32) and return a result of the parsing.
/// # Note
/// This one is easier than you think.
/// Don't think too much about the error type here in the result 👇
fn try_parse_numbers(input: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

/// This is a function to show you arrays.
/// Arrays in Rust are similar to slices, but have a fixed, compile-time
/// determined length encoded into the type itself. This function accepts
/// an array of [`i32`]s of length 5 exactly, and returns the same type.
/// It also claims ownership of the array.
///
/// Doubles the inputs and returns them.
fn double_owned_array(numbers: [i32; 5]) -> [i32; 5] {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_to_parse_numbers() {
        assert_eq!(try_parse_numbers("123"), Ok(123));
        assert_eq!(try_parse_numbers("0"), Ok(0));
        assert_eq!(try_parse_numbers("-2147483648"), Ok(i32::MIN));
        assert_eq!(try_parse_numbers("2147483647"), Ok(i32::MAX));

        assert!(try_parse_numbers("4294967295").is_err()); // u32::MAX is too big to fit into i32
        assert!(try_parse_numbers("2147483648").is_err()); // So is i32::MAX + 1, and we don't really care about the specific errors
        assert!(try_parse_numbers("what").is_err());
        assert!(try_parse_numbers("").is_err()); // Rust does not default to 0 and does not have default values (unless specified)
    }

    #[test]
    fn double_trouble_owned() {
        assert_eq!(double_owned_array([1, 2, 3, 4, 5]), [2, 4, 6, 8, 10]);
        // assert_eq!(double_owned_array([1, 2, 3, 4]), [2, 4, 6, 8]); // <- compile-error
    }
}
