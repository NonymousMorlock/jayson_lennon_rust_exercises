// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Colour {
    Green
}

impl Colour {
    fn as_string(&self) -> &'static str {
        match self {
            Colour::Green => "green"
        }
    }
}

struct ShippingBox {
    dimensions: String,
    weight: f64,
    colour: Colour
}

impl ShippingBox {
    fn new(dimensions: String, weight: f64, colour: Colour) -> Self {
        Self { dimensions, weight, colour}
    }
    fn display(&self) {
        println!("ShippingBox{{dimensions:{:?}, weight: {:?}, color: {:?}}}", self.dimensions, self.weight, self.colour.as_string())
    }
}

fn main() {
    let twenty_ton_box: ShippingBox = ShippingBox::new(String::from("2x2"), 20.0, Colour::Green);
    twenty_ton_box.display();
}
