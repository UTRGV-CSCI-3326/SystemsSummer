use std::fs::File;
use std::io::prelude::*;

struct Student {
    name: String,
    major: String,
}

impl Student {
    fn from_file(path: &str) -> Student {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let Name = lines.next().unwrap().to_string();
        let Major = lines.next().unwrap().to_string();

        Student { name, major }
    }
}

fn reading_from_file() {
    let data = Student::from_file("config.txt");
    println!("Name: {}", Student.name);
    println!("Major: {}", Student.major);
}

fn main() {
    reading_from_file();
}