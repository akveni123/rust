fn main() {
    //Write a program that counts the number of words, characters, and lines in a text input.
    //count the number of word in a string
    println!("welcome to the word counter!");

    loop {
        println!("Please enter a string to count the number of words, characters, and lines or type q to quit:");
        let input = get_input();

        let input = input.trim().to_string();
        if input.to_lowercase() == "q" {
            println!("Exiting the program. Goodbye!");
            break;
        }
        
        let word_count = input.split_whitespace().count();
        let char_count = input.chars().count();
        let line_count = input.lines().count();
        println!("Words: {}, Characters: {}, Lines: {}", word_count, char_count, line_count);
    }
}

fn get_input() ->String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
