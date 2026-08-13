mod math; // Declare the math module

fn main() {
    
    let a = 5;
    let b = 3;

    println!("Sum = {}", math::add(a, b));
    println!("Difference = {}", math::subtract(a, b));
    println!("Cube of a = {}", math::advanced::cube(a));
    println!("Square of b = {}", math::advanced::square(b));
}
