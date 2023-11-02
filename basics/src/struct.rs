enum Flavour {
    Sweet,
    Sour,
}

struct Drink {
    flavour: Flavour,
    ounces: f64,
}

fn print_food(d: Drink) {
    match d.flavour {
        Flavour::Sweet => println!("Swet drink and ounces:{:?}", d.ounces),
        Flavour::Sour => println!("Sour drink and ounces:{:?}", d.ounces),
    }
}

fn main() {

    let drink:Drink = Drink {
        flavour: Flavour::Sweet,
        ounces: 15.0  
    };
    print_food(drink);
}
