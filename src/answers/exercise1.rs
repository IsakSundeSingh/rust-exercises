pub(crate) fn uppercase_owned_string(input: String) -> String {
    input.to_uppercase()
}

pub(crate) fn lowercase_borrowed_string(input: &str) -> String {
    input.to_lowercase()
}
