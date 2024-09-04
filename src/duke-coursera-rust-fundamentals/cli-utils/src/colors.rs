//! ANSI color codes for use in terminal output. With helper functions to wrap strings in color codes.
//! # Examples:
//! ```
//! use cli_utils::colors::{red, blue};
//! let red_string = red("This is a red string");
//! let blue_string = blue("This is a blue string");
//! ```

/// Returns a string in ANSI red color codes
/// # Examples:
/// ```
/// use cli_utils::colors::red;
/// let red_string = red("This is a red string");
/// ```
pub fn red(s: &str) -> String {
    format! {"\x1b[31m{}\x1b[0m", s}
}

/// Returns a string in ANSI blue color codes
/// # Examples:
/// ```
/// use cli_utils::colors::blue;
/// let blue_string = blue("This is a blue string");
/// ```
pub fn blue(s: &str) -> String {
    format! {"\x1b[34m{}\x1b[0m", s}
}
