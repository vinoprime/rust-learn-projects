fn match_flow(some_bool: bool, some_int: i32, go: Direction) {
    match some_bool {
        true => println!("matched true"),
        false => println!("matched false"),
    }

    match some_int {
        1 => println!("matched 1"),
        2 => println!("matched 2"),
        _ => println!("matched default in switch or else"),
    }

    // match some_string {
    //     "Vino" => println!("matched Vino"),
    //     "Ragavan" => println!("matched Ragavan"),
    //     _ => println!("unmatched"),
    // }

    match go {
        Direction::UP => println!("up"),
        Direction::DOWN => println!("down"),
        Direction::LEFT => println!("left"),
        Direction::RIGHT => println!("right"),
    }
}

fn call() {
    match_flow(false, 0, Direction::UP);
    match_flow(false, 0, Direction::DOWN);
    match_flow(false, 0, Direction::LEFT);
    match_flow(false, 0, Direction::RIGHT);
}
