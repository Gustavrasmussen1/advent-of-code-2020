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

    let filename = "./dec5_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    
    let mut input:Vec<String> = Vec::new();
    let mut seat_ids:Vec<i32> = Vec::new();
    input = reader.lines().map(|x| x.unwrap()).collect();
    
    seat_ids = input.iter().map(|x| decode_seat_id(x)).collect();

    println!("MAX: {:?}",seat_ids.iter().max());

    //seat_ids = seat_ids.iter().map(|seat| seat_ids.contains(seat)).collect();
    let mut my_seat:Vec<bool> = seat_ids.iter().map(|&seat_id| !seat_ids.contains(&(&seat_id+1)) && !seat_ids.contains(&(&seat_id-1))).collect();

    let mut a = 0;
    let mut b = 0;
    for id in seat_ids.iter() {

        a = id - 1;
        b = id + 1;

        if seat_ids.contains(&a) && seat_ids.contains(&b) {
            
        }
        else{
            println!("{}", id);
        }


    }
    //println!("{:?}", my_seat);
}

fn decode_seat_id(s: &str) -> i32{

    let seat = decode_boardingpass(s);
    //let mut row = (0,127);
    //let mut column = (0,7);
    let id = seat.0 * 8 + seat.1;
    id

}

fn decode_boardingpass(s:&str) ->(i32,i32) {
    let mut row = (0,127);
    let mut column = (0,7);

    let mut seat = String::from(s);

    for c in seat.chars(){
        //println!("{}", c);
        match c.to_string().substring(0,1) {
            "F" => {
                //println!("Row from: {}-{}",row.0, row.1);
                row.1 -= (row.1 - row.0)/ 2 + 1;
                //println!("Row to: {}-{}",row.0, row.1);
            },
            "B" => {
                //println!("Row from: {}-{}",row.0, row.1);
                row.0 += (row.1-row.0)/2 + 1;
                //println!("Row to: {}-{}",row.0, row.1);
            },
            "R" => {
                //println!("Column from: {}-{}",column.0, column.1);
                column.0 += (column.1-column.0)/2 + 1;
                //println!("Column to: {}-{}",column.0, column.1);
            },
            "L" => {
                //println!("Column from: {}-{}",column.0, column.1);
                column.1 -= (column.1-column.0) / 2 + 1;
                //println!("Column to: {}-{}",column.0, column.1);
            },
            _ => {}
        }
    }

    //println!("{} {}",row.0, column.1);
    (row.0,column.1)
}
