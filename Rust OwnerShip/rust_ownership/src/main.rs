fn main() {
    ownership();
    borrowing();
    borrowing_mut();
    transfer_ownership();
    revert_ownership();
    dereference();
}




// x owns the String object "Hello, world!".
// When we assign x to y, the ownership is moved to y, and x can no longer be used to access the string.
// If you tried to use x after it was moved, Rust will throw an error because Rust prevents you from having two owners of the same value at the same time.

fn ownership() {
    let x = String::from("Hello, world!"); // x owns the String value
    let y = x; // Ownership of the String is moved from x to y

    // println!("{}", x); // This would give an error because x no longer owns the String.
    println!("{}", y); // Now y owns the String, so this works.
}


//Borrowing (Immutable Reference)
 
 
 fn borrowing() {
    let s = String::from("Hello, Rust!");
    let len = calculate_length(&s); // Borrowing s with an immutable reference

    println!("The length of '{}' is {}", s, len);
 }


fn calculate_length(s: &String) -> usize {
    s.len() // You can read the value, but can't modify it
}


//Borrowing (Mutable References)
fn borrowing_mut() {
    let mut s = String::from("Hello");
    change_string(&mut s); // Mutable borrow of s

    println!("The modified string is: {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(", Rust!"); // Modify the string through the mutable reference
}



//Transfering OwnerShip
fn transfer_ownership() {
    let a = String::from("Hello");
    let b = take_ownership(a); // Ownership of the string is moved to the function

    // println!("{}", a); // Error! a no longer owns the String
    println!("{}", b); // b owns the string now
}

fn take_ownership(s: String) -> String {
    println!("Taking ownership: {}", s);
    s // Return ownership back to the caller
}

//Reverting Ownership
fn revert_ownership(){
    let a=String::from ("Goodbye");
    let new= revert(a);
    println!("new owner is {}" ,new);

}
fn revert(s:String)->String{
    println!("Revert ownership : {}",s);
    s
}

fn dereference() {
let data = vec![1, 2, 3];
let reference = &data; // Immutable borrow

println!("Data: {:?}", *reference);
}

 
