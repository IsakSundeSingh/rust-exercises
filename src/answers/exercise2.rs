pub(crate) fn try_parse_numbers(input: &str) -> Result<i32, std::num::ParseIntError> {
    // Easy peasy! Rust infers from the return type that the
    // Ok-type of the return type of the function should be `i32`,
    // and the error type is that thing, so it infers that you want
    // to parse the string slice into an i32.
    input.parse()
}

pub(crate) fn double_owned_array(numbers: [i32; 5]) -> [i32; 5] {
    // Since we own the argument we can just make it mutable
    let mut numbers = numbers;

    // Iterate over the exclusive/mutable reference to each number
    for number in &mut numbers {
        // And double it in place
        *number *= 2;
    }

    // Returning the mutated value again, giving away ownership
    numbers
}
