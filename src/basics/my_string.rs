/*

Two commonly used typesof string
- String - owned
- &str - borrowed String String slice

Must use an owned String to store in a struct
Use &str when passing to a function


let x: &str = "this is string slice"; // like readonly
let y: String = String::from("Owned string"); // read write


Strings are autmatically borrowed
Use .to_owned() or String::from() to create an owned copy of string slice
Use an owned String when storing in a struct

*/

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name:{:?}", name);
}

/* Activity*/

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

pub fn string_main() {
    println!("Hello string");

    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("HAHAHA"),
            count: 2,
        }
    ];

    for item in receipt {
        print_name(&item.name);
        println!("count:{:?}", item.count);
    }

    let people = vec![
        Person {
            name: String::from("Google"),
            fav_color: String::from("Red"),
            age: 7,
        },
        Person {
            name: String::from("MS"),
            fav_color: String::from("blue"),
            age: 9,
        },
        Person {
            name: String::from("Linux"),
            fav_color: String::from("yellow"),
            age: 14,
        }
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
