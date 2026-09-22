pub struct Student{
    pub name: String,
    pub department: String,
}

pub struct Registry{
    pub students: Vec<Student>,
}

impl Student {
    pub fn new(name:String, department:String) -> Student {
        Student { name, department }
    }

    pub fn add_student(registry: &mut Registry, name: String, department: String) {
        let student = Student::new(name, department);
        registry.students.push(student);
    }

    pub fn find_student<'a>(registry:&'a Registry, name: &str) -> Option<&'a Student> {
        for student in &registry.students {
            if student.name == name {
                return Some(student);
            }
        }
        None
    }
}