mod car_trait;
mod method_trait;
use method_trait::*;
use car_trait::*;
use shapes::*;
mod shapes;

fn main() {
    let my_car = Car;
    //my_car.starengine();
    //my_car.honk();
    //my_car.stopengine();

    let my_methods = Methods;
    //my_methods.start();
    //my_methods.stop();

    let my_circle = Circle::new(5.0, String::from("A circle"));
    let my_rectangle = Rectangle::new(4.0, 6.0, String::from("A rectangle")); 

    println!("Circle area: {}", my_circle.area());
    println!("Circle perimeter: {}", my_circle.perimeter());
    println!("Circle description: {}", my_circle.description());

    println!("Rectangle area: {}", my_rectangle.area());
    println!("Rectangle perimeter: {}", my_rectangle.perimeter());
    println!("Rectangle description: {}", my_rectangle.description());
}
