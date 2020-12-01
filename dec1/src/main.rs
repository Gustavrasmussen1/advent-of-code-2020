use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::env;
use std::path::Path;
fn main() {
    //let dir = env::current_dir().unwrap();
    let filename= "./test_input.txt";
    //let filepath = Path::to_str();
    //let filepath = Path::new("./test_input.txt"); 
    let file = File::open(filename).expect("file not found!");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}",line.unwrap());
    }
}