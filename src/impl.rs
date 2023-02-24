/* IMPL */
struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn show_temp(&self) {
        println!("Temperature {:?}", self.degrees_f);
    }
}

/* Shipping Box */
enum Color {
    BROWN,
    RED,
    GREEN,
}

impl Color {
    fn print(&self) {
        match self {
            Color::BROWN => println!("brown"),
            Color::RED => println!("red"),
            Color::GREEN => println!("green"),
        };
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    dimensions: Dimensions,
    weight: f64,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
    }
}

fn ship_main() {
    let small_dimensions = Dimensions {
        width: 10.0,
        height: 20.0,
        depth: 5.0,
    };

    let small_box = ShippingBox::new(5.0, Color::RED, small_dimensions);
    small_box.print();

    let big_dimensions = Dimensions {
        width: 100.0,
        height: 200.0,
        depth: 10.0,
    };

    let big_box = ShippingBox::new(25.0, Color::GREEN, big_dimensions);
    big_box.print();
}

/* IMPL */
// let hot = Temperature { degrees_f: 99.69 };
// hot.show_temp();
// let cold = Temperature::freezing();
// cold.show_temp();
// cold.show_temp();
// hot.show_temp();

// ship_main();
// vec();
// vec_1();
// string_fn();
