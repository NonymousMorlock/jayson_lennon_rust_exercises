// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinates() -> (f64, f64) {
    (2122102.023, 3920399.248)
}

fn main() {
    let (_, y) = coordinates();
    if y == 5.0 {
        println!("equal to 5");
    } else if y < 5.0 {
        println!("less than 5");
    } else {
        println!("greater than 5");
    }
}
