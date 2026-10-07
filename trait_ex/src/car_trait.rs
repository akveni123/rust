pub trait Vechile {
    fn starengine(&self);
    fn stopengine(&self);
    fn honk(&self);
}

pub struct Car;

impl Vechile for Car {
    fn starengine(&self) {
        println!("Car engine started");
    }
    fn stopengine(&self) {
        println!("Car engine stopped");
    }
    fn honk(&self) {
        println!("Car honk");
    }
}