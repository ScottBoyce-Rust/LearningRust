//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` function will panic if it fails to read a line.

use std::io::{BufRead, BufReader};

pub mod colors;

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line.
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin: std::io::Stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}
