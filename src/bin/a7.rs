// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
}

fn fill(colour: Colour) {
    let colour_name = match colour {
        Colour::Red => "red",
        Colour::Green => "green",
        Colour::Blue => "blue",
        Colour::Yellow => "yellow",
    };
    println!("Filled with {:?}", colour_name);
}

fn main() {
    let prop_colour = Colour::Blue;
    fill(prop_colour);
}
