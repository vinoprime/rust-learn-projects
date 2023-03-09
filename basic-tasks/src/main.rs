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

fn main() {
    // task1();
    ms();
}
