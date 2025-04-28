fn main() {
    let length: i32 = 10;
    let width: i32 = 20;
    let height: i32 = 30;
    // let name: &str = "Rectangle";

    {
        let area: i32 = area(length, width);
        println!("The area of the rectangle is {}", area);
    }

    {
        let volume: i32 = volume(length, width, height);
        println!("The volume of the cuboid is {}", volume);
    }

    {
        let person:(&str, i32) = ("John", 30);
        println!("{} is {} years old", person.0, person.1);
        let (name, age) = person;
        println!("{} is {} years old", name, age);
    }
}

fn area(length: i32, width: i32) -> i32 {
    length * width
}

fn volume(length: i32, width: i32, height: i32) -> i32 {
    length * width * height
}