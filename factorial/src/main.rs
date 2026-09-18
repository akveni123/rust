use num_bigint::BigUint;

fn main() {
    //get input from user
    println!("Enter a number to calculate its factorial:");
    let mut input = String::new();

    let number: u32 = match read_number() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("Error in reading the number: {}", e);
            return;
        }
    };

    println!("Factorial of {} is {}", number, factorial(number));
}

fn factorial(n: u32) -> BigUint {
    if n == 0 {
        BigUint::from(1u32)
    } else {
        let mut result = BigUint::from(1u32);
        for i in 1..=n {
            result *= i;
        }
        result
    }
}

fn read_number() -> Result<u32, Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let number = input.trim().parse()?;
    Ok(number)
}
