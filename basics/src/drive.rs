#[derive(Debug, Clone, Copy)]
enum Position {
    MANAGER,
    SUPERVISOR,
    WORKER,
}

#[derive(Debug,Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp)
}

fn derive_call() {
    let me = Employee {
        position: Position::WORKER,
        work_hours: 40,
    };

    // match me.position {
    //     Position::MANAGER => println!("Manger"),
    //     Position::SUPERVISOR => println!("SUPERVISOR"),
    //     Position::WORKER => println!("WORKER")
    // }
    // println!("{:?}", me.position);
    // println!("{:?}", me);
    // println!("{:?}", me);

    print_employee(me);
    print_employee(me);
}