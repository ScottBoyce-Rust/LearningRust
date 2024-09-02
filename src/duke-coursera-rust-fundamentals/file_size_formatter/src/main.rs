// In this lab, you will enhance a size formatter application in Rust.
// Use the example code to get started and get an idea on how to
// use enum and match to handle different sizes. You are tasked
// with extending the application to allow a user to pass in a
// String representing size and unit, and then returning a
// debug representation of a struct that shows all the different
// representations in KB, MB, and GB . This is an example that takes
// an input and provides the output required:
//    $ cargo run "24 mb"
//    Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
//
// Learning Objectives:
//
//    1) Understand how to define and use enums and structs in Rust.
//    2) Practice error handling using match expressions and handling invalid input.
//    3) Gain familiarity with string parsing and formatting in Rust.
//    4) Learn how to implement the Debug trait to print debug information of a struct.
//    5) Enhance code readability by leveraging Rust's string formatting capabilities.

// ToDo:
//    1) You'll need to split the input string to capture the number and the size.
//          You can use the size (e.g. "kb") to match on how to process that number.
//    2) The struct will need to have the derive debug attribute to print it out
//    3) Use impl to extend the struct to do the work on the struct for you
//    4) Use the example code to assist you with some of the match statements and
//          transformations needed
use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl Sizes {
    fn from_bytes(bytes: u64) -> Self {
        Self::from_file_size(FileSize::Bytes(bytes))
    }

    fn from_kilobytes(kilobytes: f64) -> Self {
        Self::from_file_size(FileSize::Kilobytes(kilobytes))
    }

    fn from_megabytes(megabytes: f64) -> Self {
        Self::from_file_size(FileSize::Megabytes(megabytes))
    }

    fn from_gigabytes(gigabytes: f64) -> Self {
        Self::from_file_size(FileSize::Gigabytes(gigabytes))
    }

    fn from_file_size(file_size: FileSize) -> Self {
        match file_size {
            FileSize::Bytes(b) => Sizes {
                bytes: b,
                kilobytes: b as f64 / 1024.0,
                megabytes: b as f64 / (1024.0 * 1024.0),
                gigabytes: b as f64 / (1024.0 * 1024.0 * 1024.0),
            },
            FileSize::Kilobytes(kb) => Sizes {
                bytes: (kb * 1024.0) as u64,
                kilobytes: kb,
                megabytes: kb / 1024.0,
                gigabytes: kb / (1024.0 * 1024.0),
            },
            FileSize::Megabytes(mb) => Sizes {
                bytes: (mb * 1024.0 * 1024.0) as u64,
                kilobytes: mb * 1024.0,
                megabytes: mb,
                gigabytes: mb / 1024.0,
            },
            FileSize::Gigabytes(gb) => Sizes {
                bytes: (gb * 1024.0 * 1024.0 * 1024.0) as u64,
                kilobytes: gb * 1024.0 * 1024.0,
                megabytes: gb * 1024.0,
                gigabytes: gb,
            },
        }
    }
}

fn format_size(file_size: &FileSize) -> String {
    match file_size {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn argument_to_filesize(input: &str) -> Result<FileSize, String> {
    let input: String = input.trim().to_lowercase();

    let mut number_str = String::new();
    let mut unit_str = String::new();

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c.is_digit(10) || c == '.' {
            number_str.push(c);
        } else {
            if !c.is_whitespace() {
                unit_str.push(c);
            }
            break;
        }
    }

    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            unit_str.push(c);
        }
    }

    if unit_str.is_empty() {
        unit_str.push('b'); // Assume 'b' (bytes) if no unit is specified
    }

    let storage: f64 = match number_str.parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("Invalid file size: '{}'", number_str)),
    };

    // Match the unit to your list of accepted sizes
    let size: FileSize = match unit_str.as_str() {
        "b" => FileSize::Bytes(storage as u64),
        "kb" => FileSize::Kilobytes(storage),
        "mb" => FileSize::Megabytes(storage),
        "gb" => FileSize::Gigabytes(storage),
        _ => {
            return Err(format!(
                "Invalid size unit: '{}', accepted sizes are 'B', 'MB', 'KB', 'GB'",
                unit_str
            ))
        }
    };
    Ok(size)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: String = if args.len() == 2 {
        args[1].clone()
    } else if args.len() == 3 {
        format!("{} {}", args[1], args[2]) // If there are two arguments, concatenate them
    } else {
        println!(
            "Program only accepts one or two arguments, but received {} arguments",
            args.len() - 1 // Subtract 1 to account for the program name
        );
        return;
    };

    let file_size = argument_to_filesize(&input).unwrap();
    let result = format_size(&file_size);
    let storage_size = Sizes::from_file_size(file_size);
    println!("{}", result);
    println!("{:?}", storage_size);
}
