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
            TicketE::Backstage(price, holder) => println!("Backstage price: {:?}, holder: {:?}", price, holder),
            TicketE::Standard(price) => println!("Standard price: {:?}",price),
    }
}

fn main() {
    // derive_call();
    // call_annotations();
}
