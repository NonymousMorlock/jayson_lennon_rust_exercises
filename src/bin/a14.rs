// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

use std::convert::TryInto;

enum Colour {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Colour {
    fn as_string(&self) -> String {
        match self {
            Colour::Red => "red".to_owned(),
            Colour::Yellow => "yellow".to_owned(),
            Colour::Green => "green".to_owned(),
            Colour::Blue => "blue".to_owned(),
        }
    }

    fn at_index(index: i32) -> Self {
        match index {
            0 => Self::Red,
            1 => Self::Blue,
            2 => Self::Green,
            _ => Self::Yellow,
        }
    }
}

struct Person {
    name: String,
    age: i32,
    favourite_colour: Colour,
}

impl Person {
    fn display(&self) {
        println!("The favourite colour of {} is {}", self.name, self.favourite_colour.as_string());
    }
}

fn main() {
    let names: Vec<String> = vec![
        String::from("John"),
        String::from("Jane"),
        String::from("Doe"),
        String::from("Smith"),
    ];

    let mut people = Vec::new();

    for (i, name) in names.iter().enumerate() {
        let index: i32 = i.try_into().unwrap();
        let person = Person {
            name: name.to_owned(),
            age: (index * 4) + 2,
            favourite_colour: Colour::at_index(index),
        };
        people.push(person);
    }

    for person in &people {
       if person.age <= 10 {
           person.display()
       }
    }
}
