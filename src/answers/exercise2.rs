use std::time::Duration;

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

    // Or we could simply replace the entire function with:
    // numbers.map(|x| x * 2)
}

/// This is a struct that has an anonymous field (`0`).
/// It is a distinct type and cannot be confused for another `u32`.
/// This is called a newtype pattern.
pub(crate) struct Hertz(pub(crate) u32);

pub(crate) struct Decibel(pub(crate) u8);

pub(crate) struct Sound {
    // The volume of the sound at the sound's origin
    pub(crate) volume: Decibel,
    pub(crate) frequency: Hertz,
    pub(crate) duration: Duration,
}

/// A trait for things that can produce some noise.
pub(crate) trait Noise {
    /// The noise method returns a sound describing the noise the thing makes.
    fn noise(&self) -> Sound;
}

pub(crate) struct Dog;

// Super-generalization of some animals incoming 👇

impl Noise for Dog {
    fn noise(&self) -> Sound {
        // Dog-noises (barks) are usually around:
        // 1000-2000 Hz in frequency
        // 80-90 dB in volume
        // And have a duration of I'm guessing 500 ms
        Sound {
            volume: Decibel(85),
            frequency: Hertz(1500),
            duration: Duration::from_millis(500),
        }
    }
}

pub(crate) struct Cat;

impl Noise for Cat {
    fn noise(&self) -> Sound {
        // Cat-noises (meows) are usually around:
        // 350 Hz in frequency
        // 45 dB in volume
        // And have a duration of around 1 second
        Sound {
            volume: Decibel(45),
            frequency: Hertz(350),
            duration: Duration::from_secs(1),
        }
    }
}
