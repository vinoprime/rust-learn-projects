// Type Annotations

fn print_many(msg: &str, count: i32) {}

enum Mouse {
    LEFT_CLICK,
    RIGHT_CLICK,
    MIDDLE_CLICK,
}

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn call_annotations() {
    let num: i32 = 15;
    let a: char = 'a';
    let left_click: Mouse = Mouse::LEFT_CLICK;
    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let clicks: Vec<Mouse> = vec![Mouse::LEFT_CLICK, Mouse::RIGHT_CLICK, Mouse::MIDDLE_CLICK];

    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("other"),
        _ => println!("{:?}", n),
    };

    let flat = Discount::Flat(3);
    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("{:?}", amount),
        _ => (),
    };

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 100,
    };

    match concert {
        Ticket { price: 50, event } => println!("price = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}

enum TicketE {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn call() {
    let tickets = vec![
        TicketE::Backstage(10.0, "Vinoth".to_owned()),
        TicketE::Standard(20.0),
        TicketE::Vip(30.0, "Vip".to_owned()),
    ];

    for tkt in tickets {
        match tkt {
            TicketE::Backstage(price, holder) => {
                println!("Backstage price: {:?}, holder: {:?}", price, holder)
            }
            TicketE::Standard(price) => println!("Standard price: {:?}", price),
            TicketE::Vip(price, holder) => println!("Vip price: {:?}, price {:?}", holder, price),
        };
    }
}

// Options
// enum Option<T> {
//     some(T),
//     None,
// }

struct Customer {
    age: Option<i32>,
    email: String,
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn callOption() {
    let response = Survey {
        q1: Some(10),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("q1 ans is {:?}", ans),
        None => println!("None"),
    };

    match response.q2 {
        Some(ans) => println!("q2 ans is {:?}", ans),
        None => println!("None"),
    };

    match response.q3 {
        Some(ans) => println!("q3 ans is {:?}", ans),
        None => println!("None"),
    };
}

// Documentation
/// A favorite color.
enum Color {
    Red,
    Blue,
}

/// A piece of mail.
struct Mail {
    address: String,
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Box

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // derive_call();
    // call_annotations();
    // call();

    // let vinoth = Customer {
    //     age: Some(22),
    //     email: "Vinoth@mail.com".to_owned(),
    // };

    // let v = Customer {
    //     age: Some(35),
    //     email: "V@mail.com".to_owned(),
    // };

    // match v.age {
    //     Some(age)=> println!("customer is {:?} years old", age),
    //     None=> println!("customer age not provides")
    // };

    let b = Box::new(5);
    print!("{:?}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
