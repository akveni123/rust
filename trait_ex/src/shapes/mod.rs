pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
    pub description: String,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub description: String,
}
impl Circle{
    pub fn new(radius: f64, description: String) -> Self {
        Self { radius, description }
    }
}
impl Rectangle {
    pub fn new(width: f64, height: f64, description: String) -> Self {
        Self { width, height, description }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}
