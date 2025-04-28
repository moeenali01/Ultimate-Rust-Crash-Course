fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    
    inspect(&arg);
    change(&mut arg);
    eat(arg);
    changee(&mut arg);
}





fn inspect(s:&String){
   if s.ends_with("s"){
       println!("{} ends with 's'",s);
    }else{
        println!("{} does not end with 's'",s);
    }
}

fn change(s:&mut String){
    if !s.ends_with("s"){
        s.push_str("s");
    }
    println!("The new string is {}",s);
}

fn eat(s:String){
    if s.starts_with("b") && s.contains("a"){
        println!("YUM");
    }
}

fn changee(s: &mut String) {
    *s = "sparkly".to_string();
    println!("The new string is {}", s);
}
