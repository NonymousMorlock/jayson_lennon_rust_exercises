// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Orange,
    Lemon,
    Mango,
    Pineapple,
    Vanilla,
    Butterscotch,
    Mint,
    Strawberry,
    Apple,
}

struct Drink {
    flavour: Flavour,
    ounces: f64,
}

fn flavour_to_string(flavour: &Flavour) -> String {
    match flavour {
        Flavour::Orange => "orange".to_string(),
        Flavour::Lemon => "lemon".to_string(),
        Flavour::Mango => "mango".to_string(),
        Flavour::Pineapple => "pineapple".to_string(),
        Flavour::Vanilla => "vanilla".to_string(),
        Flavour::Butterscotch => "butterscotch".to_string(),
        Flavour::Mint => "mint".to_string(),
        Flavour::Strawberry => "strawberry".to_string(),
        Flavour::Apple => "apple".to_string()
    }
}

fn print_drink(drink: &Drink) {
    let flavour = flavour_to_string(&drink.flavour);
    println!("Drink(flavour: {:?}, ounces: {:?})", flavour, drink.ounces);
}

fn main() {
    let pina_colada = Drink{flavour: Flavour::Pineapple, ounces: 2.5};
    print_drink(&pina_colada);
}
