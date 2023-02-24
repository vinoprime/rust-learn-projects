struct LineItme {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name : {:?}", name);
}

fn string_fn() {
    let receipt = vec![
        LineItme {
            name: "cerea".to_owned(),
            count: 1,
        },
        LineItme {
            name: String::from("another way"),
            count: 5,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("name : {:?}, count : {:?}", item.name, item.count);
    }
}


/* String  */
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print_person(data: &str) {
    println!("{:?}", data);
}

fn person_fn() {

    let people = vec![
        Person {
            name: "vinoth".to_owned(),
            fav_color: "red".to_owned(),
            age: 9,
        },
        Person {
            name: String::from("Rag"),
            fav_color: String::from("yellow"),
            age: 24,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print_person(&person.name);
            print_person(&person.fav_color)
        }
    }
}
