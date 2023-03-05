/* Ownership */
enum Light {
    BRIGHT,
    DULL,
}

fn display_light(light: &Light) {
    match light {
        Light::BRIGHT => println!("Bright"),
        Light::DULL => println!("Dull"),
    };
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("Pages {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Rating {:?}", book.rating);
}

fn call() {
    let dull = Light::DULL;
    display_light(&dull);
    display_light(&dull);

    let book = Book {
        pages: 5,
        rating: 9,
    };
    display_page_count(&book);
    display_rating(&book);
}
