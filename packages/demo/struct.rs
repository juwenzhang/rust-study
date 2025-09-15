struct Student {
    name: String,
    age: u8,
}

impl Student {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
}

fn main() {
    let mut student = Student::new("张三".to_string(), 18);
    println!("姓名: {}", student.get_name());
    println!("年龄: {}", student.get_age());
    student.set_name("李四".to_string());
    student.set_age(20);
    println!("姓名: {}", student.get_name());
    println!("年龄: {}", student.get_age());
}