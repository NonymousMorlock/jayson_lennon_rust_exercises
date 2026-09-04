// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

use std::io;
use std::num::ParseIntError;

#[derive(Clone, Copy)]
struct Student {
    id: i32,
    name: &'static str,
    locker_id: Option<i32>,
}

impl Student {
    fn as_string(&self) -> String {
        let mut display: String = format!("Hello {}! ", self.name);
        match self.locker_id {
            Some(locker_id) => {
                display.push_str(format!("Locker with id [{:?}] is yours.", locker_id).as_str());
                display
            }
            None => {
                display.push_str(
                    "You have no locker assigned yet. Reach out to the admin for assistance.",
                );
                display
            }
        }
    }
}

fn main() {
    let students: Vec<Student> = vec![
        Student {
            id: 1,
            name: "John Doe",
            locker_id: Some(23394281),
        },
        Student {
            id: 2,
            name: "Jane Dean",
            locker_id: None,
        },
        Student {
            id: 3,
            name: "Marston Lane",
            locker_id: None,
        },
    ];

    println!("Student id: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong.");

    let parsed_id: Result<i32, ParseIntError> = input.trim().parse();

    let student_id: i32 = match parsed_id {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };

    for student in &students {
        if student_id == student.id {
            println!("{}", student.as_string());
            return;
        }
    }
    println!("Student not found");
}
