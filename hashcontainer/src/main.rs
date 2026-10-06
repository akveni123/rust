use std::io;
mod functions;
use functions::count_words;

fn main() {
    
    let mut input = String::new();
    println!("Enter a string to count the words:");
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    
    let word_count = count_words(&input);
    println!("{:?}", word_count);
}
