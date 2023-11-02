enum Color {
    RED,
    BLUE,
    GREEN,
}

fn disp_color(color: Color) {
    match color {
        Color::RED => println!("red color"),
        Color::BLUE => println!("blue color"),
        Color::GREEN => println!("green color"),
    }
}

fn main() {
    disp_color(Color::GREEN);
}
