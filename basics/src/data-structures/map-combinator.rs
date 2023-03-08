fn num() -> Option<i32> {}

fn word() -> Option<String> {}

fn main() {
    let plus_one = match num() {
        Some(num) => Some(num + 1),
        None => None,
    };

    let plus_one = num().map(|num| num + 1);

    let word_len = word().map(|word| word.len()).map(|len| len * 2);
}

struct User {
    id: i32,
    name: String,
}

fn find_user() -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "Katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let user_name = "Vino";
    let user = find_user(user_name).map(|id| User {
        id,
        name: user_name.to_owned(),
    });
}
