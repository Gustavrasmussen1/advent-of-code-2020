use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::collections::HashMap;
use std::ops::{Bound, RangeBounds};

// Source: https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11 - User carlomilanesi
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}
// Source: https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11 - User carlomilanesi
impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}


fn main() {
    
    let filename = "./dec8_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut input:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    // Part 1
    let mut accumulator:i32 = 0;
    let mut visited_operations: Vec<String> = Vec::new();
    let mut current_operation:i32 = 0;

    let terminated = false;
    loop {

        if current_operation as usize >= input.len() {
            println!("Terminated!");
            //let terminated = true;
            break;
        }
        if visited_operations.contains(&current_operation.to_string()) {
            //println!("Been here!");
            break;
        }


        visited_operations.push(current_operation.to_string());
        let (op,x) = parse_operation(&input[current_operation as usize]);

        accumulator += x;
        current_operation += op;
        // add to vector of used lines

        // Part 1
        //println!("{}", accumulator);
    }

    // Part 2
    let terminated = false;
    for n in 0..= &input.len()-1 {

        let mut accumulator:i32 = 0;
        let mut visited_operations: Vec<String> = Vec::new();
        let mut current_operation:i32 = 0;
    
        
        // Flip operation

        input[n] = flip_operation(&input[n]);

        // Brute force here
        loop {

            if current_operation as usize >= input.len() {
                println!("Acc: {}", accumulator);
                let terminated = true;
                break;
            }
            if visited_operations.contains(&current_operation.to_string()) {
                break;
            }
    
            visited_operations.push(current_operation.to_string());
            let (op,x) = parse_operation(&input[current_operation as usize]);
    
            accumulator += x;
            current_operation += op;
        }

        if terminated {
            println!("{}", accumulator);
        }

        // Flip back
        input[n] = flip_operation(&input[n]);
    }


    
}

fn parse_operation(operation: &String) -> (i32, i32) {
    // Takes in Operation in String and returns next operation and integer to accumulator 

    let words: Vec<&str> = operation
    .split_whitespace()
    .collect();
    
    let op = words[0];
    let x = words[1].parse::<i32>().unwrap();

    let mut result = (0,0);
    match op {
        "nop" => { result = (1,0);},
        "acc" => { result = (1,x);},
        "jmp" => { result = (x,0);},
        _ => {}
    }
    
    result
}

fn flip_operation(operation: &String) -> String {

    let words: Vec<&str> = operation
    .split_whitespace()
    .collect();
    
    let mut op = words[0];
    let x = words[1];
    let mut op_flip = op;
    match op {
        "nop" => {op_flip = "jmp";},
        "jmp" => {op_flip = "nop";},
        _ => {}
    }

    op_flip.to_string() + " " +  x

}