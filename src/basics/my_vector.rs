/*
Multiple pieces of data
- must be the same type
Used for lists of information
can add, remov and traverse the entries


syntax:

let my_num = vec![1,2,3];

let mut my_mun = Vec::new();
my_num.push(1);
my_num.pop();
my_num.len();

let two = my_num[1];


for num in my_num {
    println!("{?}", num);
}

*/

struct Test {
    score: i32,
}

pub fn vectors_main() {
    println!("Hello Vectors");

    let my_scores = vec![
        Test { score: 23 },
        Test { score: 33 },
        Test { score: 43 },
        Test { score: 53 }
    ];

    for test in my_scores {
        println!("Score: {:?}", test.score);
    }

    /* activity */
    let my_num = vec![10, 20, 30, 40];

    for num in &my_num {
        match num {
            30 => println!("Thirty"),
            _ => println!("Others : {:?}", num),
        }
    }

    println!("no of elements : {:?}", my_num.len());
}
