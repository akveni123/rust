fn main() {
    
    let x = 5;
    let y = x;
    //ex_1(x,y);
    //ex_2();
    //ex_3_ownership_transfer();

    let mut text = String::from("Hello");
    mutate(&mut text);
    println!("{text}");
}

fn ex_1(x: i32, y: i32){
    println!("x: {}", x);
    println!("y: {}", y);
}

fn ex_2() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone s1 to s2  ;
    println!("s1 = {}",s1);
}

fn ex_3_ownership_transfer() {

    let name = String ::from("Apple");
    let name = call_fruit(name);

    println!("I am {name}");
}
fn call_fruit(name: String)->String {
    println!("I am {}", name);
    name
}
fn Borrowing() {
    let text = String::from("Hello");
    let borrowed = &text;
    println!("{text}");
}

fn mutate(text : &mut String) {
    text.push_str(" Don");
}

fn ex_4 () {
    let mut numbers = vec![1, 2, 3, 4, 5];

    let first = &numbers[0];
    //numbers.push(6); // This line will cause a compile-time error because we are trying

    println!("The first number is: {}", first);
}

fn ex_5() {
    
    let mut name = String::from("Name");

    let r1 = &name; // immutable borrow
    let r2 = &name; // immutable borrow
    //let r3 = &mut name; // mutable borrow

   // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

pub mod ex_6 {

    pub struct Config {
        pub app_name: String
    }

    pub fn get_app_name(config: &Config) -> &String {
        &config.app_name
    }

}

mod mini_project {
    
    struct Student{
        name: String,
        department: String,
    }

    struct Registry{
        students: Vec<Student>,
    }

    impl Student {
        fn new(name:String, department:String) -> Student {
            Student { name, department }
        }

       fn add_student(registry: &mut Registry, name: String, department: String) {
            let student = Student::new(name, department);
            registry.students.push(student);
        }

        fn find_student<'a>(registry: &'a Registry, name: &str) -> Option<&'a Student> {
            for student in &registry.students {
                if student.name == name {
                    return Some(student);
                }
            }
            None
        }
    }
}

