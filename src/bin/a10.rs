// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

enum Size {
    Big { message: String },
    Small { message: String },
}

fn print_size_message(size: Size) {
    match size {
        Size::Big { message } => println!("{message}"),
        Size::Small { message } => println!("{message}"),
    }
}

fn main() {
    let chunk_size: i32 = 2000;
    let size_type: Size = if chunk_size > 100 {
        Size::Big {message: String::from("It's big")}
    } else {
        Size::Small {message: String::from("It's small")}
    };
    print_size_message(size_type);
}
