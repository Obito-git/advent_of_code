use std::io::{self, BufRead};

/// Reads a file path from stdin and returns the file contents.
pub fn read_input() -> String {
    let stdin = io::stdin();
    let path = stdin
        .lock()
        .lines()
        .next()
        .expect("No input provided")
        .expect("Failed to read line");
    std::fs::read_to_string(path.trim()).expect("Failed to read file")
}
