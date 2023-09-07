use std::path::Path;

/// Reads the entire content of a file and returns an iterator over each line.
pub fn read_lines(path: &Path) -> impl Iterator<Item = String> {
    std::fs::read_to_string(path)
        .expect("Invalid path name for inputs")
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>()
        .into_iter()
}
