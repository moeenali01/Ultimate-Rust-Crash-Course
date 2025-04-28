use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter your name:");

    // Read the user input
    io::stdin()
        .read_line(&mut input) // Reads the line of input and appends it to `input`
        .expect("Failed to read line");

    // Remove any trailing newline or whitespace
    let input = input.trim();

    println!("Hello, {}", input);
}
