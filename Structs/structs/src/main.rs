fn main() {
    book();
   setcolour();
}


struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn book() {
    let book1 = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        pages: 180,
    };

    println!("Book: {}, Author: {}, Pages: {}", book1.title, book1.author, book1.pages);
}


//Tuple Struct
struct Color(u8, u8, u8);

fn setcolour() {
    let red = Color(255, 0, 0);

    println!("Red: {}, Green: {}, Blue: {}", red.0, red.1, red.2);
}

