mod util;
use util::temp_utl;

fn main() {
    
    let mut temperature = temp_utl::initsize();

    for temp in temperature {
        println!("Temperature: {}", temp);
    }
}
