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

fn call() {
    /* Expressions */
    let my_num = 3;
    let is_lt_5 = if my_num < 5 {
        true;
    } else {
        false;
    };

    let access_level = Access::GUEST;
    let can_access_file = match access_level {
        Access::ADMIN => true,
        _ => false,
    };
    println!("{:?}", can_access_file);
    
    let value = 100;
    let is_gt_100 = value > 100;
    print_msg(is_gt_100);
}
