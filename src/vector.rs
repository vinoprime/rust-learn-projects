/* Vector */
struct Test {
    score: i32,
}

fn vec() {
    let my_scores = vec![Test { score: 10 }, Test { score: 20 }, Test { score: 30 }];
    for test in my_scores {
        println!("{:?}", test.score);
    }
}

fn vec_1() {
    let my_numbers = vec![10, 20, 30, 40, 50];

    for num in &my_numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        };
    }
    println!("{:?}", my_numbers.len());
}