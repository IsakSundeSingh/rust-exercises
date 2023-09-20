fn uppercase_owned_string(input: String) -> String {
    crate::answers::exercise1::uppercase_owned_string(input)
}

#[cfg(test)]
mod tests {
    // Star imports are fine when done like this, in my opinion.
    use super::*;

    #[test]
    fn can_uppercase() {
        assert_eq!(
            uppercase_owned_string("hello, there!".into()),
            String::from("HELLO, THERE!")
        );

        assert_eq!(
            uppercase_owned_string("denne håndterer jo æ ø og å".into()),
            String::from("DENNE HÅNDTERER JO Æ Ø OG Å")
        );
    }
}
