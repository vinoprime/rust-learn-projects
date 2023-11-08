enum Mouse {
    LeftClick,
    RightClick,
    Scroll(i32),
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percentage(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}
pub fn enums_main() {
    println!("hi Enum")
}
