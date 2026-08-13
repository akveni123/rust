fn main() {
    
    println!("Welcome to the temperature converter!");

    loop {
        println!("Please enter the temperature you want to convert (e.g 32F, 100C) or type q to quit:");
        let input = get_input();

        if input.to_lowercase() == "q" {
            println!("Exiting the program. Goodbye!");
            break;
        }

        match parse_input(&input) {
            (value, 'C') | (value, 'c') => {
                let fahrenheit = celsius_to_fahrenheit(value);
                println!("{}C is equal to {}F", value, fahrenheit);
            }
            (value, 'F') | (value, 'f') => {
                let celsius = fahrenheit_to_celsius(value);
                println!("{}F is equal to {}C", value, celsius);
            }
            _ => {
                println!("Invalid unit. Please enter a valid temperature (e.g 32F, 100C).");
            }
        }
    }
}

fn get_input()->String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_input(input:&str) ->(f64,char) {
    let input = input.trim();
    let last_char = input.chars().last().unwrap_or(' ');
    let value_str = &input[..input.len().saturating_sub(1)];
    let value = value_str.parse::<f64>().unwrap_or(0.0);
    (value, last_char)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
