// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for i in 1..5 {
        numbers.push(i * 10);
    }

    for number in &numbers {
       println!("{}", if *number != 30 { number.to_string() } else { "thirty".into() });
    }

    println!("Total number of elements: {:?}", numbers.len())
}
