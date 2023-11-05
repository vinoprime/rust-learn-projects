enum Color {
    Brown,
    Red,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

pub fn impl_main() {
    println!("IMPL");

    let small = Dimensions {
        width: 10.0,
        height: 11.0,
        depth: 2.0,
    };

    let small_box = ShippingBox::new(5.0, Color::Red, small);
    small_box.print();

    // let sp1 = ShippingBox
}
