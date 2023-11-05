fn coordinate() -> (i32, i32) {
    return (50, 5);
}

pub fn tupple() {
    println!("main_tupple");
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("name", 20);
    println!("{:?}, {:?}", name, age);

    // Data Management using tuple
    let (x, y) = coordinate();

    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5")
    }
}
