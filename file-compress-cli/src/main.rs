extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

use std::io::Read;

fn compress() {
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}

fn main() {
    println!("Hello, world!");

    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let path = std::path::Path::new("a.pdf");
    // Open the file in read mode
    let mut file = File::open(path).unwrap();

    let mut buffer = Vec::new();
    file.read(&mut buffer).unwrap();

    let file_contents = String::from_utf8(buffer).unwrap();

    println!("{:?}", file_contents);

    // compress();
}
