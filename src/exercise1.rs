/// Accepts a vector of the lines in the exercise and returns a vector of integers
fn convert_to_integers(lines: Vec<String>) -> Vec<u8> {
    lines.into_iter().map(|x| x.parse().unwrap()).collect()
}

/// Accepts a vector of the lines in the exercise and returns their sum
fn sum_lines(lines: Vec<String>) -> u8 {
    convert_to_integers(lines).into_iter().sum()
}

/// Accepts a vector of the lines in the exercise and returns their sum
/// # Bonus!
/// Try implementing this as with [sum_lines], but using a different method than that one!
fn sum_lines_bonus(lines: Vec<String>) -> u8 {
    #![allow(clippy::unnecessary_fold)]
    convert_to_integers(lines)
        .into_iter()
        .fold(0, |x, acc| x + acc)
}

/// Accepts a vector of the lines in the exercise and returns a vector of the even numbers
fn keep_evens(lines: Vec<String>) -> Vec<u8> {
    convert_to_integers(lines)
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn lines() -> Vec<String> {
        crate::parsing::read_lines(Path::new("inputs/exercise_1.txt")).collect()
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
