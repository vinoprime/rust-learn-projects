/*--------------------------------*/
struct Book {
    pages: i32,
    ratings: i32,
}

fn disp_page_count(book: &Book) {
    println!("{:?}", book.pages);
}

fn disp_ratings(book: &Book) {
    println!("{:?}", book.ratings);
}
/*--------------------------------*/

struct GroceryItem {
    qty: i32,
    id: i32,
}


fn disp_qty(g: &GroceryItem) {
    println!("{:?}", g.qty);
}

fn disp_id(g: &GroceryItem) {
    println!("{:?}", g.id);
}

pub fn owners() {
    println!("ooo");

    let book: Book = Book {
        pages: 5,
        ratings: 9,
    };

    disp_page_count(&book);
    disp_ratings(&book);

    let g_item = GroceryItem {
        qty: 10,
        id:1000
    };

    disp_qty(&g_item);
    disp_id(&g_item);
    
}
