fn printMsg(msg: bool) {
    
     match msg {
        true => println!("its big"),
        _ => println!("its small")
    };
}
pub fn expressions() {
    println!("Expressions");

    let my_num = 3;
    // let is_lt_5 = if my_num < 5 { true } else { false };
    let is_lt_5 = my_num < 5;
    println!("{:?}", is_lt_5);

    // Requirements:
    // Print "its big" if a var is > 100
    // Print "its small" if a var is <= 100

    println!("Activity");

    let s = 1000;
    let is_gt_100 = s > 100;

    printMsg(is_gt_100);
}
