use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("V", 21);
    people.insert("Ed", 13);
    people.insert("will", 21);
    people.insert("mak", 31);
    people.insert("sak", 51);

    people.remove("V");

    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age),
        None => println!("Not Found"),
    }

    for (person, age) in people.iter() {
        println!("Person = {:?} , {:?}", person, age)
    }

    for person in people.keys() {
        println!("Person = {:?}", person);
    }

    for age in people.values() {
        println!("age = {:?}", age);
    }
}

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    locker.insert(
        2,
        Contents {
            content: "new stuff".to_owned(),
        },
    );

    for (lock_num, content) in lockers.iter() {
        println!("lock_no {:?} and conetnt : {:?}", lock_num, content);
    }
}

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut total_stock = 0;

    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        let stock_count = if qty == &0 {
            "Out of stock".to_owned()
        } else {
            format!("{:?}", qty);
        };
        println!("item={:?} and qty={:?}", item, stock_count);
    }

    println!("total={:?}", total_stock);
}
