// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

struct Furniture {
    name: String,
    quantity: i32,
}

fn main() {
    let furniture_database: HashMap<i32, Furniture> = HashMap::from([
        (
            1,
            Furniture {
                name: "Chair".to_owned(),
                quantity: 5,
            },
        ),
        (
            2,
            Furniture {
                name: "Bed".to_owned(),
                quantity: 3,
            },
        ),
        (
            3,
            Furniture {
                name: "Table".to_owned(),
                quantity: 2,
            },
        ),
        (
            4,
            Furniture {
                name: "Couch".to_owned(),
                quantity: 0,
            },
        ),
    ]);

    let mut total_number_of_items: i32 = 0;

    for furniture in furniture_database.values() {
        if furniture.quantity < 0 {
            println!(
                "Something went wrong. Furniture with name {} has a negative quantity - {}",
                furniture.name, furniture.quantity
            );
            return;
        }
        println!(
            "{} -- {}",
            furniture.name,
            if furniture.quantity > 0 {
                furniture.quantity.to_string()
            } else {
                "out of stock".to_owned()
            }
        );
        total_number_of_items += furniture.quantity;
    }

    println!("Total number of items in stock: {}", total_number_of_items);
}
