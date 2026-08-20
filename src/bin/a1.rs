// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
   println!("First Name: {:?};\nLast Name: {:?}", first_name(), last_name());
}

fn first_name() -> String {
   "John".to_string()
}

fn last_name() -> String {
   "Smith".to_string()
}
