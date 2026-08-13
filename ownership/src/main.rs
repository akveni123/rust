fn main() {
    
    
    let s:&str = "Hello, world!";
    let s1:String = String::from("Hello, world!");

    let s2 = s1;;

    let number = 2;
    let mut number2 = number;

    println!("Number: {}", s1);

    //print_string(&s1);
    
}

fn print_string(s: &String) {
    println!("String: {}", s);
}

fn movestring(s: String) {
    println!("String: {}", s);
}   

