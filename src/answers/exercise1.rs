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
