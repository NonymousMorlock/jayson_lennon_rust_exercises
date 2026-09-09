// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::num::ParseIntError;

enum PowerOption {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerOption {
    fn from_string(value: &String) -> Result<Self, String> {
        let option: Self = match value.to_lowercase().as_str() {
            "off" => Self::Off,
            "sleep" => Self::Sleep,
            "reboot" => Self::Reboot,
            "shutdown" => Self::Shutdown,
            "hibernate" => Self::Hibernate,
            _ => return Err("Invalid option".to_owned()),
        };

        Ok(option)
    }

    fn action_message(&self) -> String {
        match self {
            Self::Off => "Turning off display".to_owned(),
            Self::Sleep => "Putting computer to sleep".to_owned(),
            Self::Reboot => "Restarting".to_owned(),
            Self::Shutdown => "Shutting down".to_owned(),
            Self::Hibernate => "Hibernating".to_owned(),
        }
    }

    fn from_i32(value: &i32) -> Result<Self, String> {
        let string_value: &str = match value {
            0 => "off",
            1 => "sleep",
            2 => "reboot",
            3 => "shutdown",
            4 => "hibernate",
            _ => return Err("Invalid option".to_owned()),
        };
        Self::from_string(&string_value.to_owned())
    }

    fn variants() -> HashMap<i32, String> {
        HashMap::from([
            (0, "off".to_owned()),
            (1, "sleep".to_owned()),
            (2, "reboot".to_owned()),
            (3, "shutdown".to_owned()),
            (4, "hibernate".to_owned()),
        ])
    }

    fn display(&self) {
        println!("{}", self.action_message())
    }
}

fn main() {
    let mut buffer: String = String::new();

    let variants: HashMap<i32, String> = PowerOption::variants();

    let variants_length: i32 = variants.len().try_into().unwrap();

    println!("Options:");

    for i in 0..variants_length {
        println!("{}. {}", i + 1, variants.get(&i).unwrap())
    }
    println!("\nEnter an option number: ");

    io::stdin()
        .read_line(&mut buffer)
        .expect("Something went wrong");

    let parsed_buffer: Result<i32, ParseIntError> = buffer.trim().parse();

    let option: i32 = match parsed_buffer {
        Ok(option) => option,
        Err(_) => {
            println!("Invalid option");
            return;
        }
    } - 1;

    match PowerOption::from_i32(&option) {
        Ok(option) => option,
        Err(message) => {
            println!("{message}");
            return;
        }
    }
    .display()
}
