struct Student {
    name: String,
    age: u8,
    grade: Vec<f32>,
}

impl Student {
    fn new(name: String, age: u8) -> Self {
        Self{
            name: name.to_string(),
            age,
            grade: vec![],
        }
    }
}

fn main() {
    let mut students: Vec<Student> = vec![];
    (&mut students).push(Student::new("Donatia".to_string(),21));
    (&mut students).push(Student::new("Gragathe".to_string(),21));
    (&mut students).push(Student::new("Lugras".to_string(),21));
    (&mut students).push(Student::new("Ashinta".to_string(),20));

    for student in &students {
        println!("{:}", student.name);
    }
}