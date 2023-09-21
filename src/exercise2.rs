#![allow(unused_variables)]

use std::time::Duration;

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

/// This is how you create a generic function and generic argument.
/// This function accepts a type which it knows nothing about.
/// If you know about pure functional programming, you know that there is
/// really nothing you can do with a generic type.
///
/// **Not even printing**
/// Printing is often a hacked-in solution because values are objects
/// and objects can be printed etc., but values in Rust are just values,
/// and methods are only function pointers, so there is no information on
/// how to print or do anything with a value unless you use a function that
/// can act on it. This function needs to return a `T` so it has to just
/// return the input. Generics kind of seem useless right about now, huh?
fn generics<T>(t: T) -> T {
    // println!("{t}"); // <- compile error
    t
}

/// # Introducing... Traits!
///
/// Traits are Rust's answer to ad-hoc polymorphism, or in plain English:
/// how to constrain and add "behaviour" to generic types. Traits are
/// what makes generics useful.
///
/// Traits in Rust are similar to interfaces in languages like C# and Java,
/// but better. So more like Haskell's typeclasses, yet not fully that
/// powerful.
///
/// Traits are introduced using the `trait` keyword and then the name, followed
/// by a block describing the trait. An empty body means the trait is called a
/// "marker trait". It just marks a value. Rust internally uses this for a lot
/// of different tricks.
trait Traits {}

/// This is a struct that has an anonymous field (`0`).
/// It is a distinct type and cannot be confused for another `u32`.
/// This is called a newtype pattern.
struct Hertz(u32);

struct Decibel(u8);

struct Sound {
    // The volume of the sound at the sound's origin
    volume: Decibel,
    frequency: Hertz,
    duration: Duration,
}

/// A trait for things that can produce some noise.
trait Noise {
    /// The noise method returns a sound describing the noise the thing makes.
    fn noise(&self) -> Sound;
}

struct Dog;

// Super-generalization of some animals incoming 👇

impl Noise for Dog {
    fn noise(&self) -> Sound {
        // Dog-noises (barks) are usually around:
        // 1000-2000 Hz in frequency
        // 80-90 dB in volume
        // And have a duration of I'm guessing 500 ms
        todo!()
    }
}

struct Cat;

impl Noise for Cat {
    fn noise(&self) -> Sound {
        // Cat-noises (meows) are usually around:
        // 350 Hz in frequency
        // 45 dB in volume
        // And have a duration of around 1 second
        todo!()
    }
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

    fn meow_and_bark() -> (Sound, Sound) {
        let cat = Cat;
        let dog = Dog;

        let meow = cat.noise();
        let bark = dog.noise();
        (meow, bark)
    }

    #[test]
    fn noisy_cats_meow_with_lower_frequency_than_dogs() {
        // I looked it up (ish, I skimmed it):
        // Cat-paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7000907/
        // Dog-paper: https://article.sciencepublishinggroup.com/pdf/10.11648.j.ijmea.20140201.14.pdf
        let cat = Cat;
        let dog = Dog;

        let meow = cat.noise();
        let bark = dog.noise();

        // Trait-methods can also be called as free-standing functions:
        let meow = Noise::noise(&cat);
        let bark = Noise::noise(&dog);

        // The methods can also be called in a "universal function-call syntax" (UFCS):
        let meow = <Cat as Noise>::noise(&cat);
        let bark = <Dog as Noise>::noise(&dog);

        assert!(meow.frequency.0 < bark.frequency.0); // We need to use `.0` to extract the anonymous field of the `Hertz`-newtype
    }

    #[test]
    fn noisy_dogs_are_louder_than_cats() {
        let (meow, bark) = meow_and_bark();
        assert!(bark.volume.0 > meow.volume.0);
    }

    #[test]
    fn noisy_dogs_and_cats_make_noises_longer_than_0_seconds() {
        let (meow, bark) = meow_and_bark();
        assert!(bark.duration > Duration::from_secs(0));
        assert!(meow.duration > Duration::from_secs(0));
    }
}
