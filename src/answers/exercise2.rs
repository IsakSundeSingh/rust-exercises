pub(crate) fn try_parse_numbers(input: &str) -> Result<i32, std::num::ParseIntError> {
    // Easy peasy! Rust infers from the return type that the
    // Ok-type of the return type of the function should be `i32`,
    // and the error type is that thing, so it infers that you want
    // to parse the string slice into an i32.
    input.parse()
}
