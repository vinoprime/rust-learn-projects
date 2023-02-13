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

/* Display a msg based on the value of a boolean variable when variable is true display "Hello", when variable is false display "Bye" */
fn display_hello(true_false: bool) {
    if true_false == true {
        println!("Hello");
    } else {
        println!("Bye");
    }
}
/* Display a msg based on the value of a boolean variable when variable is true display "Hello", when variable is false display "Bye" */

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

fn control_flow() {
    let age = 25;
    if age >= 21 {
        println!("ok to purchase");
    } else {
        println!("cannot purchase")
    }
}

fn match_flow(some_bool: bool, some_int: i32, go: Direction) {
    match some_bool {
        true => println!("matched true"),
        false => println!("matched false"),
    }

    match some_int {
        1 => println!("matched 1"),
        2 => println!("matched 2"),
        _ => println!("matched default in switch or else"),
    }

    // match some_string {
    //     "Vino" => println!("matched Vino"),
    //     "Ragavan" => println!("matched Ragavan"),
    //     _ => println!("unmatched"),
    // }

    match go {
        Direction::UP => println!("up"),
        Direction::DOWN => println!("down"),
        Direction::LEFT => println!("left"),
        Direction::RIGHT => println!("right"),
    }
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

/* DEMO Expressions*/
enum Access {
    ADMIN,
    MANAGER,
    USER,
    GUEST,
}

fn print_msg(gt_100: bool) {
    match gt_100 {
        true => println!("big"),
        _ => println!("small"),
    };
}

/* Ownership */
enum Light {
    BRIGHT,
    DULL,
}

fn display_light(light: &Light) {
    match light {
        Light::BRIGHT => println!("Bright"),
        Light::DULL => println!("Dull"),
    };
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("Pages {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Rating {:?}", book.rating);
}

/* IMPL */
struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn show_temp(&self) {
        println!("Temperature {:?}", self.degrees_f);
    }
}

/* Shipping Box */
enum Color {
    BROWN,
    RED,
    GREEN,
}

impl Color {
    fn print(&self) {
        match self {
            Color::BROWN => println!("brown"),
            Color::RED => println!("red"),
            Color::GREEN => println!("green"),
        };
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    dimensions: Dimensions,
    weight: f64,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
    }
}

fn ship_main() {
    let small_dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 5.0,
    };

    let small_box = ShippingBox::new(5.0, Color::RED, small_dimensions);
    small_box.print();

    let big_dimensions = Dimensions {
        width: 100.0,
        height: 200.0,
        depth: 10.0,
    };

    let big_box = ShippingBox::new(25.0, Color::GREEN, big_dimensions);
    big_box.print();
}

/* Vector */
struct Test {
    score: i32,
}

fn vec() {
    let my_scores = vec![Test { score: 10 }, Test { score: 20 }, Test { score: 30 }];
    for test in my_scores {
        println!("{:?}", test.score);
    }
}

fn vec_1() {

    let my_numbers = vec![10, 20, 30, 40, 50];

    for num in &my_numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        };
    }
    println!("{:?}", my_numbers.len());
}

fn main() {
    // basics();
    // basics_loop();
    // control_flow();
    // match_flow(false, 0, Direction::UP);
    // match_flow(false, 0, Direction::DOWN);
    // match_flow(false, 0, Direction::LEFT);
    // match_flow(false, 0, Direction::RIGHT);

    // let cereal = GroceryItem {
    //     stock: 10,
    //     price: 10.5,
    // };
    // println!("Stock: {:?}", cereal.stock);

    // let sweet = Drink {
    //     flavor: Flavour::SWEET,
    //     fluid_oz: 6.0,
    // };
    // print_drink(sweet);

    // let fruity = Drink {
    //     flavor: Flavour::FRUITY,
    //     fluid_oz: 5.0,
    // };
    // print_drink(fruity);

    // Tuples
    // let coord = (2, 3);
    // let (x, y) = (2, 3);
    // let (name, age) = ("Vino", 34);
    // println!("{:?} {:?}", coord.0, coord.1);
    // println!("{:?} {:?}", x, y);
    // println!("{:?} {:?}", name, age);

    /* Expressions */
    // let my_num = 3;
    // let is_lt_5 = if my_num < 5 {
    //     true;
    // } else {
    //     false;
    // };

    // let msg = match my_num {
    //     1 => "Hello",
    //     _ => "bye",
    // };

    // let access_level = Access::GUEST;
    // let can_access_file = match access_level {
    //     Access::ADMIN => true,
    //     _ => false,
    // };
    // println!("{:?}", can_access_file);

    // let value = 100;
    // let is_gt_100 = value > 100;
    // print_msg(is_gt_100);

    // let dull = Light::DULL;
    // display_light(&dull);
    // display_light(&dull);

    // let book = Book {
    //     pages: 5,
    //     rating: 9,
    // };
    // display_page_count(&book);
    // display_rating(&book);

    /* IMPL */
    // let hot = Temperature { degrees_f: 99.69 };
    // hot.show_temp();
    // let cold = Temperature::freezing();
    // cold.show_temp();
    // cold.show_temp();
    // hot.show_temp();

    // ship_main();
    // vec();
    vec_1();
}
