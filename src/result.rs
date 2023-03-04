enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("Unable to find")
    }
}

fn call() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("Sound data located"),
        Err(e) => println!("Error: {:?}", e),
    };
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuchMenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuchMenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    // ? automatically performs match operation
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(());
}

//   Problem
/*
   Requirements
       * Determine if a customer is able to make  a restricted purchase
       * Restricted purchases require that the age of the customer is atlease 21
   Notes
       * Use a struct to store at lease the age of a customer
       * Use a function to determine if a customer can make a restricted purchase
       * Return a result from the function
       * The err variant should detail the reason why they cannot make a purchase
*/

struct Customer {
    age: i32,
}

fn try_purchase(customer: &Customer) -> Result((), String) {
    if customer.age > 21 {
        Err("atleast must be 21 years old");
    } else {
        Ok(());
    }
}

fn manin() {
    // let choice = get_choice("mainmenu");
    pick_choice("start");

    let vino = Customer { age: 20 };
    let purchased = try_purchase(&vino);
    println!("{:?}", purchased);
}

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_access(emp: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    };

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid Position".to_owned()),
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let attempt_access = try_access(emp)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Position::Manger,
        status: Status::Active,
    };

    match print_access(&manager) {
        Err(e) => println!("access denied: {:?}"),
        _ => (),
    }
}
