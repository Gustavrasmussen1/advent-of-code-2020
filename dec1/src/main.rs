use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename= "./test_input.txt";
    let file = File::open(filename).expect("file not found!");

    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        println!("{}",line.unwrap());
    }
}