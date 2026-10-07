pub trait methods_default {
    fn start(&self);
    fn stop(&self){
        self.start();
        println!("Stopping...");
    }
}

pub struct Methods;

impl methods_default for Methods {
    fn start(&self) {
        println!("Hello..!...Starting...");
    }
}

fn main() {
    let my_methods = Methods;
    my_methods.start();
    my_methods.stop();
}