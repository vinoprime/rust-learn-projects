use chrono::prelude::*;
use std::io;

// Simple task in Rust that involves taking user input and printing it back to the console:
fn task1() {
    println!("Please Enter your name:");

    let mut name = String::new();

    io::stdin().read_line(&mut name);

    println!("Hello, {:?}", name.trim());
}

// Write a program that takes two numbers as input and outputs the sum of those numbers.
fn ms() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Please enter a number:");
    io::stdin().read_line(&mut a);

    println!("Please enter b number:");
    io::stdin().read_line(&mut b);

    let a1: i32 = a.trim().parse().expect("Invalid number");
    let b1: i32 = b.trim().parse().expect("Invalid number");

    let c = a1 + b1;
    println!("the sum of you entered numbers is  : {:?}", c);
}

// Write a program that reads  integers, one integer per line.
// The program should output the result to the console.
fn task2() {
    let mut input = String::new();

    for i in 1..5 {
        println!("Please Enter numbers");
        let mut tempStr = String::new();
        io::stdin().read_line(&mut tempStr).expect("Invalid number");
        input.push_str(&tempStr.trim());
    }
    println!("{:?}", input);
}

//Write a program that asks the user for their name and age, and then outputs a message with their name and the year they will turn 100 years old.
//For example, if the user enters "Alice" and "25", the program should output something like: "Hello, Alice! You will turn 100 in the year 2096."
fn task3() {
    println!("Started...");

    let mut name = String::new();
    let mut age = String::new();
    println!("What is your name:");
    io::stdin().read_line(&mut name).expect("Enter valid");
    println!("What is your age:");
    io::stdin().read_line(&mut age).expect("Enter valid number");

    let a1: i32 = age.trim().parse().expect("Not a vaild number");

    // Calculating the year the user will turn 100
    let currentYear = Utc::now().year() as i32;
    let birthYear = (currentYear - a1);
    let yr_of_100th = birthYear + 100;
    print!("{:?}", yr_of_100th);
}

fn main() {
    // task1();
    // ms();
    // task2();
    task3();
}
