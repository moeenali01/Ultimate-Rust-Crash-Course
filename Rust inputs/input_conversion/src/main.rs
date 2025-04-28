use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter a number:");

    // Read input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert input to a number (i32)
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("You entered: {}", number);
}
