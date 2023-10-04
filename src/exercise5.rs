#![allow(unused_variables)]

/// The trait `Iterator` is really common and ingrained in Rust
/// It is used for iterating over values and is implemented for nearly
/// every standard-library type you'd need it for. It is also required to make
/// for-loops work. We can create our own Iterator-trait like this:
trait MyIterator {
    /// The type that is returned for each iteration
    type Item;

    /// Advances the iterator and returns the next value, or [`None`]
    /// if the iterator is depleted.
    fn next(&mut self) -> Option<Self::Item>;

    /// Trait methods can have a default-implementation, that use other
    /// methods that are only described in the trait as their implementation.
    /// This is how implementing only [`Self::Item`] and [`Self::next`] is
    /// necessary to get all the nice Iterator-methods.
    ///
    /// Here is a method that returns folds over the iterator,
    /// only using the [`Self::next`]-method.
    fn fold<Accumulator, F>(mut self, init: Accumulator, mut f: F) -> Accumulator
    where
        // This just means the type has a compile-time determined size (e.g. 10 bytes, and not unknown).
        Self: Sized,
        F: FnMut(Accumulator, Self::Item) -> Accumulator,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
}

/// If you have a list of [`Result<T, E>`][`Result`]s, you can use `collect()` to
/// see if any of them failed:
///
/// ```
/// let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
///
/// let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
///
/// // gives us the first error
/// assert_eq!(Err("nope"), result);
///
/// let results = [Ok(1), Ok(3)];
///
/// let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
///
/// // gives us the list of answers
/// assert_eq!(Ok(vec![1, 3]), result);
/// ```
struct IteratorsAndCollectsAreVeryUsefulButThisStructIsNot;

/// Accepts a vector of the lines in the exercise and returns a vector of integers
fn convert_to_integers(lines: Vec<String>) -> Vec<u8> {
    todo!()
}

/// Accepts a vector of the lines in the exercise and returns their sum
fn sum_lines(lines: Vec<String>) -> u8 {
    todo!()
}

/// Accepts a vector of the lines in the exercise and returns their sum
/// # Bonus!
/// Try implementing this as with [`sum_lines`], but using a different method than that one!
fn sum_lines_bonus(lines: Vec<String>) -> u8 {
    todo!()
}

/// Accepts a vector of the lines in the exercise and returns a vector of the even numbers
fn keep_evens(lines: Vec<String>) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn lines() -> Vec<String> {
        crate::parsing::read_lines(Path::new("inputs/number_inputs.txt")).collect()
    }

    #[test]
    fn converts_to_integers() {
        assert_eq!(
            convert_to_integers(lines()),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    #[test]
    fn lines_sums_correctly() {
        assert_eq!(sum_lines(lines()), 55)
    }

    #[test]
    fn sums_lines_in_mysterious_ways() {
        assert_eq!(sum_lines(lines()), sum_lines_bonus(lines()))
    }

    #[test]
    fn retains_evens() {
        assert_eq!(keep_evens(lines()), vec![2, 4, 6, 8, 10])
    }
}
