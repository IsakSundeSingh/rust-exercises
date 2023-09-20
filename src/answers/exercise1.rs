pub(crate) fn uppercase_owned_string(input: String) -> String {
    input.to_uppercase()
}

pub(crate) fn lowercase_borrowed_string(input: &str) -> String {
    input.to_lowercase()
}

pub(crate) fn find_largest_in_slice(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().max()
}

pub(crate) struct Square {
    length: u32,
}

impl Square {
    pub(crate) fn new(side_length: u32) -> Self {
        Self {
            length: side_length,
        }
    }

    pub(crate) fn area(&self) -> u32 {
        self.length * self.length
    }

    pub(crate) fn perimeter(&self) -> u32 {
        self.length * 4
    }
}

pub(crate) enum BoringColor {
    Red,
    Green,
    Blue,
}

impl BoringColor {
    pub(crate) fn hex(&self) -> String {
        match self {
            BoringColor::Red => "FF0000",
            BoringColor::Green => "00FF00",
            BoringColor::Blue => "0000FF",
        }
        .into()
    }
}

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

    fn rgb_hex(&self) -> String {
        use Color::*;
        match &self.color_kind {
            Red => "FF0000".into(),
            Green => "00FF00".into(),
            Blue => "0000FF".into(),
            Custom(rgb) => rgb.into(),
        }
    }

    fn rgba_hex(&self) -> String {
        let Self { color_kind, alpha } = self;

        use Color::*;
        let rgb: &str = match &color_kind {
            Red => "FF0000",
            Green => "00FF00",
            Blue => "0000FF",
            Custom(rgb) => rgb,
        };

        // Format the alpha value with leading zeros (the `0`) and up to two in
        // length (the `2`) as hexadecimal (the `x`).
        // This should be familiar if you've used Python.
        // See more in the [`std::fmt`]-module: https://doc.rust-lang.org/std/fmt/#usage
        format!("{rgb}{alpha:02x}")
    }

    fn is_transparent(&self) -> bool {
        self.alpha == 0
    }
}
