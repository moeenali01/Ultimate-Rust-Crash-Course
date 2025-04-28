use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter two numbers:");

    // Read two inputs
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Split the input into words (space-separated) and parse them
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace() // Split the input by whitespace (space, tab, etc.)
        .filter_map(|s| s.parse().ok()) // Convert each split part into i32
        .collect();

    if numbers.len() == 4 {
        println!("You entered: {}, {}, {} and {}", numbers[0], numbers[1], numbers[2], numbers[3]);
    } else {
        println!("Please enter exactly two numbers!");
    }
}
