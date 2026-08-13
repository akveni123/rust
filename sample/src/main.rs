fn main() {
    println!("Hello, world!");

    let s:&str = "Hello, world!";
    let s1:String = String::from("Hello, world!");
    

    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        }

        if counter % 2 == 0 {
            println!("Counter is even: {}", counter);
        }

        println!("Counter: {}", counter);
    }

    
}
