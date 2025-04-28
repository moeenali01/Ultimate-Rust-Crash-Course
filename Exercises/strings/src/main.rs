fn main() {

    let mut greeting = String::new();
    greeting.push_str("Hello");
    greeting.push(' ');
    greeting.push_str("World");
    println!("{}", greeting);

    let s1: String = get_string();
    println!("This is s1: {}", s1);

    let s2: String = String::from("world");
    let s3: String = send_get_string(s2);
    

    println!("This is s3: {}", s3);

    {
    let greeting2 = String::from("Hello, world!");
    println!("{}", greeting2); 
    }

    //&Str - immutable Strings
    {
        let my_str: &str = "Hello, World!";  

        println!("{}", my_str);  
    }

    //Converting String to &Str
    {
        let mut my_string = String::from("Hello, World!");

        let my_str = &my_string;
        println!("{}", my_str);
        my_string.push_str(" How are you?");
        println!("{}", my_string);
        
    }

    //Converting &Str to String
    {
        let my_str = "Hello, World!";

        let my_string = my_str.to_string();
        println!("{}", my_string);
    }

    //String Concentration
    {
        let hello = String::from("Hello");
        let world = String::from("World");
        
        let greeting = hello + ", " + &world;  
        println!("{}", greeting); 
    }
    
    //String Concatenation with format

    {
        let hello = String::from("Hello");
        let world = String::from("World");
        
        let greeting = format!("{}, {}!", hello,  world +" Everyone");  
        println!("{}", greeting);  
    }
    
    //Slicing Strings
    {
        let my_string = String::from("Hello, World!");
    
        let hello = &my_string[0..8];
        println!("{}", hello);  
    }

    //String Iterations

     {
        let my_string = String::from("Rust");
    
        for c in my_string.chars() {
            println!("{}", c);  
        }
    }

    

    //String Searching
    
     {
        let my_string = String::from("Hello, Russt!");
    
        if my_string.contains("Rust") {
            println!("Found 'Rust' in the string!");  
        }
        else {
            println!("Did not find 'Rust' in the string.");  
        }

    }  
    

}

fn get_string() -> String {
    let new_string = String::from("hello");
    return new_string;
}

fn send_get_string(received_string: String) -> String {
    return received_string;
}