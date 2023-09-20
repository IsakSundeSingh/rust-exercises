#![allow(unused_variables)]

fn uppercase_owned_string(input: String) -> String {
    todo!()
}

fn lowercase_borrowed_string(input: &str) -> String {
    todo!()
}

/// Returns the largest element of the slice, if any.
/// Returns [`None`] when the given slice is empty,
/// or the largest value wrapped in [`Some`] otherwise.
/// It needs to return a reference to the number because
/// we do not own the numbers and we reference back to the
/// original number.
/// (For those that know about [`Copy`] types, we'll get to that)
fn find_largest_in_slice(numbers: &[i32]) -> Option<&i32> {
    todo!()
}

/// TODO: This struct is incomplete and needs to store some dimension to calculate it's area
/// Look inside the answers module for the answers of these exercises.
struct Square {}

impl Square {
    fn new(side_length: u32) -> Self {
        todo!()
    }

    fn area(&self) -> u32 {
        todo!()
    }

    /// The perimeter of a square (and any irregular polygon) is the sum of each of the sides
    fn perimeter(&self) -> u32 {
        todo!()
    }
}

enum BoringColor {
    Red,
    Green,
    Blue,
}

impl BoringColor {
    /// Returns the hexadecimal string representation of the value in RGB (6 characters)
    fn hex(&self) -> String {
        todo!()
    }
}

/// # Bonus
/// Kind of difficult exercise, so jump to the bottom of the tests and unignore the test if you want to try.
struct AdvancedColor {
    color_kind: Color,
    alpha: u8,
}

enum Color {
    Red,
    Green,
    Blue,
    /// For those that don't really like red, green or blue.
    /// Assume this is constructed with a valid hexadecimal RGB string of 6 characters
    Custom(String),
}

impl AdvancedColor {
    fn opaque_white() -> Self {
        Self {
            color_kind: Color::Custom("FFFFFF".into()),
            alpha: 255,
        }
    }

    /// Returns a hex color like [`BoringColor`], without the alpha
    fn rgb_hex(&self) -> String {
        todo!()
    }

    /// Returns a hex color of 8 characters in RGBA format
    fn rgba_hex(&self) -> String {
        todo!()
    }

    /// Whether or not a color is fully transparent or not
    /// E.g. 0 is fully transparent, everything else is not.
    fn is_transparent(&self) -> bool {
        todo!()
    }
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

    #[test]
    fn slice_it_down_the_middle() {
        assert_eq!(find_largest_in_slice(&[1, 2, 3, 4, 5]), Some(&5));
        assert_eq!(find_largest_in_slice(&[1]), Some(&1));
        assert_eq!(find_largest_in_slice(&[i32::MIN]), Some(&i32::MIN));
        assert_eq!(find_largest_in_slice(&[i32::MAX]), Some(&i32::MAX));
        assert_eq!(find_largest_in_slice(&[]), None);
    }

    #[test]
    fn squares_are_easy() {
        assert_eq!(Square::new(10).area(), 100);
        assert_eq!(Square::new(2).area(), 4);
        assert_eq!(Square::new(8).area(), 64);
        assert_eq!(Square::new(0).area(), 0);
    }

    #[test]
    fn perimeters_of_squares_are_easy() {
        assert_eq!(Square::new(0).perimeter(), 0); // Is this really a square? Really makes you think
        assert_eq!(Square::new(2).perimeter(), 8);
        assert_eq!(Square::new(1).perimeter(), 4);
        assert_eq!(Square::new(3).perimeter(), 12);
    }

    #[test]
    fn feel_the_graphic_designer_inside_you() {
        assert_eq!(BoringColor::Red.hex().to_uppercase(), "FF0000");
        assert_eq!(BoringColor::Green.hex().to_uppercase(), "00FF00");
        assert_eq!(BoringColor::Blue.hex().to_uppercase(), "0000FF");
    }

    #[test]
    #[ignore = "Bonus exercise"]
    fn i_know_photoshop() {
        assert_eq!(
            AdvancedColor::opaque_white().rgb_hex().to_lowercase(),
            String::from("ffffff")
        );
        assert_eq!(
            AdvancedColor::opaque_white().rgba_hex().to_lowercase(),
            String::from("ffffffff")
        );

        let transparent_red = AdvancedColor {
            color_kind: Color::Red,
            alpha: 0,
        };
        assert!(transparent_red.is_transparent());

        let opaque_green = AdvancedColor {
            color_kind: Color::Green,
            alpha: 0,
        };
        assert!(opaque_green.is_transparent());

        let somewhere_in_between_pink = AdvancedColor {
            color_kind: Color::Custom("FF00FF".into()),
            alpha: 127,
        };

        assert_eq!(
            somewhere_in_between_pink.rgb_hex().to_lowercase(),
            String::from("ff00ff")
        );
        assert_eq!(
            somewhere_in_between_pink.rgba_hex().to_lowercase(),
            String::from("ff00ff7f")
        );

        let somewhere_in_between_pink = AdvancedColor {
            color_kind: Color::Custom("FF00FF".into()),
            alpha: 1,
        };

        assert_eq!(
            somewhere_in_between_pink.rgb_hex().to_lowercase(),
            String::from("ff00ff")
        );
        assert_eq!(
            somewhere_in_between_pink.rgba_hex().to_lowercase(),
            String::from("ff00ff01")
        );
    }
}
