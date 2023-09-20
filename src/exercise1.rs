#![allow(unused_variables)]

fn uppercase_owned_string(input: String) -> String {
    todo!()
}

fn lowercase_borrowed_string(input: &str) -> String {
    crate::answers::exercise1::lowercase_borrowed_string(input)
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

    #[test]
    fn can_lowercase_and_create_new_string() {
        assert_eq!(lowercase_borrowed_string("SHOUT"), String::from("shout"));
        assert_eq!(
            lowercase_borrowed_string("THIS WAS NOT AS HARD AS YOU THOUGHT"),
            String::from("this was not as hard as you thought")
        );
    }
}
