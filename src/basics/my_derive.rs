/*
If we want to print the whole enum/struct we need to use derive


available => #[derive(Debug, Clone, Copy)]

#[derive(Debug)]
enum Position {
    Manager,
    Worker,
}

#[derive(Debug)]
struct Employee {
    position : Position,
    work_hours:i64
}


let me = Employess {
    position : Position::Worker,
    work_hours: 40,
}

println!("{:?}", me.position);

println!("{:?}", me);

// only works with Copy
fn print(e Employee){
    println!("{:?}", e);
}

*/


/*
type Annotaions

let num: i32 = 15;
let a: char ='a';
let num: Vec<i32> = vec![1,2,3];
let num: Vec<char> = vec!['1','2','3'];


*/ 

pub fn derive_main() {
    println!("Hi Derive");
}
