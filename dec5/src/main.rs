use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
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

    let filename = "./dec5_test_input.txt";
    decode_boardingpass(filename);

}

fn decode_boardingpass(s: &str){

    let filename = s;
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}",line.unwrap());
        }
        
    }
