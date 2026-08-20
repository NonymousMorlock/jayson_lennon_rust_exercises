// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display_sum(add, 1, 2)
}

fn display_sum(function: fn(n1: i32, n2: i32) -> i32, param_1: i32, param_2: i32) {
    println!("{:?} + {:?} = {:?}", param_1, param_2, function(param_1, param_2))
}

fn add(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}
