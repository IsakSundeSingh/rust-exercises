pub(crate) fn convert_to_integers(lines: Vec<String>) -> Vec<u8> {
    lines.into_iter().map(|x| x.parse().unwrap()).collect()
}

pub(crate) fn sum_lines(lines: Vec<String>) -> u8 {
    convert_to_integers(lines).into_iter().sum()
}

pub(crate) fn sum_lines_bonus(lines: Vec<String>) -> u8 {
    #![allow(clippy::unnecessary_fold)]
    convert_to_integers(lines)
        .into_iter()
        .fold(0, |x, acc| x + acc)
}

pub(crate) fn keep_evens(lines: Vec<String>) -> Vec<u8> {
    convert_to_integers(lines)
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect()
}
