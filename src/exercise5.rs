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

/// 👇 this is how you say an impl block is generic over a type
impl<T> MyIterator for Option<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // Don't worry too much about this implementation, it simply changes the &mut self reference to `None` and returns the thing &mut self originally pointed to as an owned value
        std::mem::take(self)
    }
}

/// A function that accepts a generic type parameter that implements `MyIterator`
fn generic_over_t_with_bound_shorthand<T: MyIterator>(t: T) {}

/// A function that accepts a generic type parameter that implements `MyIterator` using a where clause is the same as the above
fn generic_over_t_with_bound_where_clause<T>(t: T)
where
    T: MyIterator,
{
}

/// A function that accepts an argument that implements [`MyIterator`], but it only knows that about the argument.
/// It does not know it's type and is dissimilar to the function above, but in practice is the same.
fn function_accepts_only_something_that_impls_my_iterator(t: impl MyIterator) {}

/// A function that returns a generic type parameter that implements `MyIterator`.
/// This is a really weird function if you think about it because it has to
/// return a generic type that the caller knows, but it itself doesn't know
/// about the type, so it can't really return it?
/// Don't worry too much about it, just showing some syntax around traits here.
fn function_that_returns_generic_type<T: MyIterator>() -> T {
    unimplemented!("Deliberately unimplemented, don't try to implement this")
}

/// A function that accepts a generic type parameter that implements `MyIterator`
fn function_that_returns_something_that_impls_my_iterator<Whatever>(
) -> impl MyIterator<Item = Whatever> {
    // Hehe just wrote this to make it compile,
    // don't worry about it if you think it's difficult,
    // or think about it if you want to know,
    // or just ask me 😅
    None
}

/// Collect is a super useful method on iterators
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
