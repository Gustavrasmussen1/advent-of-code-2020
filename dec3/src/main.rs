use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "./dec3_input.txt";
    
    let mut r:i32 = 3;
    let mut d:i32  = 1;

    // Part 1
    let path = get_path(filename,r,d);
    println!("Part 1: ");
    println!("{}", path.matches("#").count());
    //println!("{}",path);

    // Part 2
    println!("Part 2: ");
    let mut result = path.matches("#").count();

    r = 1;
    d = 1;
    let path = get_path(filename,r,d);
    result *= path.matches("#").count();
    //println!("{}", path.matches("#").count());
    //println!("{}",path);

    r = 5;
    d = 1;
    let path = get_path(filename,r,d);
    result *= path.matches("#").count();

    r = 7;
    d = 1;
    let path = get_path(filename,r,d);
    result *= path.matches("#").count();

    r = 1;
    d = 2;
    let path = get_path(filename,r,d);
    result *= path.matches("#").count();

    println!("{}",result);
}

fn get_path(s: &str, _right: i32, _down: i32) -> String{
    let filename = s;
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut next_line = _down;
    let mut path = String::from("");

    let mut x:i32 = 0;

    for line in reader.lines() {
        if counter == next_line {
            
            let mut a = String::from(line.unwrap());
            x += _right;
            //let x = (counter * _right) as usize;

            while a.len() < (x + 1)  as usize {
                let b = a.clone();
                a = a + &b;
            }
            
            let c = String::from(a.chars().nth(x as usize).unwrap());
            path += &c;
            next_line += &_down;
        }
        counter += 1;



    }
    
    path
}