//! # Exercises!
//! A small library with advent of code-like exercises to test your Rust skills.
//! Try running the tests using `cargo test`.
//! If you want to run a single module's tests or a specific test,
//! just enter a unique prefix such as `cargo test -- exercise1`.
//! You can use `cargo test --quiet` or `cargo test -q` to not print every
//! test case result if you want the output shorter.
//!
//! # Note
//! All functions you implement do not expect error-handling unless specified,
//! so unwrapping is safe and ok to do (if done as expected ;)).
//! They are only expected to work with respect to the corresponding exercise text file.
//!
//! # Cheating
//! If you'd like to see a possible solution or just make a test pass, there
//! are solutions for every exercise in the [`answers`]-module.
//! To use that answer in you function, you could just replace the function
//! of the body like this:
//! ```ignore
//! // Inside module `x`
//! fn some_function_to_implement(thing: Thing) -> Thang {
//!     crate::answers::exercisex::some_function_to_implement(thing)
//! }
//! ```

#![allow(dead_code)]
#![forbid(unsafe_code)]

mod answers;
mod exercise1;
mod exercise_x_iterators;
mod parsing;
