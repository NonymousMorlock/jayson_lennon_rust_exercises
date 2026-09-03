// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

use crate::Ticket::{Backstage, Standard, Vip};

enum Ticket {
    Backstage { holder_name: String, price: f64 },
    Vip { holder_name: String, price: f64 },
    Standard { price: f64 },
}

fn main() {
    let mut tickets: Vec<Ticket> = Vec::new();
    for i in 1..=3 {
        let ticket: Ticket;
        if i == 1 {
            ticket = Backstage {
                holder_name: "John Doe".to_owned(),
                price: 1999.99,
            };
        } else if i == 2 {
            ticket = Vip {
                holder_name: "Albert Donny Einstein".to_owned(),
                price: 2999.99,
            };
        } else {
            ticket = Standard { price: 99.99 };
        }
        tickets.push(ticket);
    }

    for ticket in &tickets {
        match ticket {
            Backstage { holder_name, price } => println!("{}::Backstage::£{}", holder_name, price),
            Vip { holder_name, price } => println!("{}::Vip::£{}", holder_name, price),
            Standard { price } => println!("Brokie::Standard::£{}", price),
        }
    }
}
