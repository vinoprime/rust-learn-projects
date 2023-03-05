/*  Display the result of the sum of two numbers */

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}
fn display_result(result: i32) {
    println!("{:?}", result);
}

/*  Display the result of the sum of two numbers */

// Basics

fn basics() {
    let sum = 2 + 3;
    let value = 10 - 5;
    let division = 10 / 2;
    let mult = 5 * 5;

    let rem = 6 % 3;
    let rem2 = 6 % 4;
}



fn basics_loop() {
    //Infinite Loop
    let mut x = 0;
    loop {
        if x == 5 {
            break;
        }
        println!("{:?}", x);
        x = x + 1;
    }

    // while x <= 4 {
    //     println!("Inside while");
    // }
}


enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

enum Flavour {
    SPARKLING,
    SWEET,
    FRUITY,
}
struct Drink {
    flavor: Flavour,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavour::SPARKLING => println!("Sprinkling"),
        Flavour::SWEET => println!("SWEET"),
        Flavour::FRUITY => println!("Fruity"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}
// Basics




/* Display a msg based on the value of a boolean variable when variable is true display "Hello", when variable is false display "Bye" */
fn display_hello(true_false: bool) {
    if true_false == true {
        println!("Hello");
    } else {
        println!("Bye");
    }
}
/* Display a msg based on the value of a boolean variable when variable is true display "Hello", when variable is false display "Bye" */



fn control_flow() {
    let age = 25;
    if age >= 21 {
        println!("ok to purchase");
    } else {
        println!("cannot purchase")
    }
}



fn call(){
    let cereal = GroceryItem {
        stock: 10,
        price: 10.5,
    };
    println!("Stock: {:?}", cereal.stock);

    let sweet = Drink {
        flavor: Flavour::SWEET,
        fluid_oz: 6.0,
    };
    print_drink(sweet);


    let fruity = Drink {
        flavor: Flavour::FRUITY,
        fluid_oz: 5.0,
    };
    print_drink(fruity);

    let msg = match my_num {
        1 => "Hello",
        _ => "bye",
    };

}