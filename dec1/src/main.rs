use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename= "./dec1_input_a.txt";

    let input:Vec<i32> = input_to_i32vector(filename);


    // Advent of Code Day 1 - Part 1
    for a in &input {
        for b in &input {
            if check_sum_of_two(a,b,2020){
                println!("{} * {} = {}",a,b, a*b);
            }       
        }
    }

    // Advent of Code Day 1 - Part 2

    for a in &input {
        for b in &input {
            for c in &input {
                if check_sum_of_three(a,b,c,2020){
                    println!("{} * {} * {} = {}",a,b,c, a * b * c)
                }
            } 
        }
    }


}

fn input_to_i32vector(s: &str) -> Vec<i32> {
    let filename = s;
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut v: Vec<i32> = Vec::new();

    for line in reader.lines() {
        
        let a: i32 = line.unwrap().parse().expect("unable to parse");
        v.push(a);
    }

    // Return the Vec<i32>
    v

}

fn check_sum_of_two(a: &i32, b: &i32, x: i32) -> bool {
    if a + b == x {
        true
    }
    else {
        false
    }
}

fn check_sum_of_three(a: &i32, b: &i32, c:&i32, x: i32) -> bool {
    if a + b + c == x{
        true
    }
    else {
        false
    }
}