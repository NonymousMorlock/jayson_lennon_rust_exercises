// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

use std::io;
use std::num::ParseIntError;

struct Customer {
    id: i32,
    name: String,
    age: i32,
}

struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    minimum_age: Option<i32>,
}

struct Order {
    customer: Customer,
    product: Product,
}

impl Product {
    fn display(&self) -> () {
        let name_and_id: String = format!("{} - id: {:?}", self.name, self.id);
        match &self.minimum_age {
            Some(age) => {
                println!("{} - Age Restriction: {} or older", name_and_id, age)
            }
            None => println!("{}", name_and_id),
        }

        match &self.description {
            Some(description) => println!("\n\n{description}"),
            None => (),
        }
    }
}

fn validate_order(order: &Order) -> Result<(), String> {
    let minimum_age: i32 = match order.product.minimum_age {
        Some(age) => age,
        None => return Ok(()),
    };
    if order.customer.age < minimum_age {
        return Err(format!(
            "You must be {} or older to complete the purchase.",
            minimum_age
        ));
    }
    Ok(())
}

fn get_customer(customer_id: i32) -> Result<Customer, String> {
    let customer_database: Vec<Customer> = vec![
        Customer {
            id: 1,
            name: "John Doe".to_owned(),
            age: 21,
        },
        Customer {
            id: 2,
            name: "Jane Doe".to_owned(),
            age: 34,
        },
        Customer {
            id: 3,
            name: "John Smith".to_owned(),
            age: 12,
        },
        Customer {
            id: 4,
            name: "Jane Crate".to_owned(),
            age: 18,
        },
    ];

    for customer in customer_database {
        if customer.id == customer_id {
            return Ok(customer);
        }
    }
    Err("User not found!".to_owned())
}

fn get_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Vodka".to_owned(),
            minimum_age: Some(21),
            description: None,
        },
        Product {
            id: 2,
            name: "Marmite Yeast Extract".to_owned(),
            minimum_age: None,
            description: None,
        },
        Product {
            id: 3,
            name: "McVitie's Digestive Biscuits".to_owned(),
            minimum_age: None,
            description: None,
        },
        Product {
            id: 4,
            name: "Gordon's London Dry Gin".to_owned(),
            minimum_age: Some(18),
            description: None,
        },
    ]
}

struct DisplayProductsParams {
    customer_age: Option<i32>,
}

fn display_products(params: DisplayProductsParams) -> () {
    let products: Vec<Product> = get_products();
    println!("------------------------------MENU------------------------------");
    if products.is_empty() {
        println!("\n---------------------------NO PRODUCTS---------------------------\n");
        println!("\n-----------------------------------------------------------------\n");
    }
    for product in products {
        match (params.customer_age, product.minimum_age) {
            (Some(customer_age), Some(minimum_age)) if customer_age < minimum_age => continue,
            _ => {
                println!("\n-----------------------------------------------------------------\n");
                product.display();
                println!("\n-----------------------------------------------------------------\n");
            }
        }
    }
}

fn get_product(product_id: i32) -> Result<Product, String> {
    let products: Vec<Product> = get_products();
    for product in products {
        if product.id == product_id {
            return Ok(product);
        }
    }
    Err("Product not found!".to_owned())
}

fn main() {
    println!("\nEnter your id: ");

    let mut customer_id_input: String = String::new();

    io::stdin()
        .read_line(&mut customer_id_input)
        .expect("Something went wrong.");

    let customer_id: i32 = match customer_id_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };

    let customer_result: Result<Customer, String> = get_customer(customer_id);
    let customer: Customer = match customer_result {
        Ok(customer) => customer,
        Err(message) => {
            println!("{message}");
            return;
        }
    };

    println!("\nWelcome aboard {}", customer.name);

    println!();
    display_products(DisplayProductsParams {
        customer_age: Some(customer.age),
    });
    println!("\nEnter the product id: ");

    let mut product_id_input: String = String::new();

    io::stdin()
        .read_line(&mut product_id_input)
        .expect("Something went wrong.");

    let parsed_product_id: Result<i32, ParseIntError> = product_id_input.trim().parse();

    let product_id: i32 = match parsed_product_id {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };

    let product_result: Result<Product, String> = get_product(product_id);
    let product: Product = match product_result {
        Ok(product) => product,
        Err(message) => {
            println!("{message}");
            return;
        }
    };

    let order = Order { customer, product };

    let validation_result: Result<(), String> = validate_order(&order);

    match validation_result {
        Ok(_) => {
            println!();
            println!("Product:");
            order.product.display();
            println!("Order Successful.");
        }
        Err(message) => {
            println!("{message}");
            return;
        }
    }
}
