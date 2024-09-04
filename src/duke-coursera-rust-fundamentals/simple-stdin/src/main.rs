use std::io;

fn main() {
    let mut input = String::new();
    let mut input_trim = input.trim();
    while input_trim.to_lowercase() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to exit): ");
        io::stdin()
            .read_line(&mut input)
            .expect("stdin line read error");
        input_trim = input.trim();
        println!("You entered: {input_trim}");
    }
    println!("Program Complete");
}
