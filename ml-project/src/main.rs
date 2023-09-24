use csv::Reader;
use linfa::Dataset;
use ndarray::{Array, Array1, Array2};
use std::fs::File;

fn get_dataset()  {
    let mut reader = Reader::from_path("./src/heart.csv").unwrap();

}

fn main() {
    println!("Hello, world!");
    get_dataset()
}
