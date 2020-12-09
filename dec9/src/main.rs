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
    let filename = "./dec9_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut input:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut input_i:Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    //println!("{:?}", valid_numbers(&input_i[0..25],&403));

    let mut invalid_num = 0;
    for (i,num) in input_i.iter().enumerate(){

        if i == input_i.len()-25 {break}
        /*
        if !valid_numbers(&input_i[i..i+25],&input_i[i+25]){
            //println!("{:?}", &input_i[i..i+25]);

            println!("{} {}",i, &input_i[i+26]);
            
        }
        */

        //println!("{:?}", &input_i[i..i+25]);
        let target = &input_i[i+25];
        let mut b = false;

        for x in &input_i[i..i+25] {
            
            for y in &input_i[i..i+25]{
                
                if x+y == *target && x != y {
                    b = true;
                    break;
                }
            }
            if b {break}
        }
        if !b {
            //println!("{}", target);
            invalid_num = *target;
            break;

        }

        //println!("{} {:?}",&input_i[i+6], Some(valid_numbers(&input_i[i..i+5],&input_i[i+6])));
    }
    //println!("{}", invalid_num);

    //let cont_num = vec![];
    for x in 2..input_i.len() {

        for y in 0..input_i.len()-x {
            let tmp_sum = &input_i[y..x+y].iter().sum::<i64>();
            //println!("{:?}", &input_i[y..x+y].iter().sum::<i64>());
            if *tmp_sum == invalid_num { 
                println!("{:?}",  (input_i[y..x+y].iter().max().unwrap() +input_i[y..x+y].iter().min().unwrap()));
                println!("{:?}", &input_i[y..x+y]);
                //println!("{}", invalid_num);
                //let cont_sum = &input_i[y..x+y];
                break;
            }
        }
    }
    //let my_vec = vec![138, 133, 281, 137, 134, 154, 148, 150, 161, 179, 211, 181, 182, 215, 203, 217, 232, 233, 237, 234, 241, 267, 254, 256, 257];
    //println!(" {}", valid_numbers(&my_vec,&403));

}

fn valid_numbers(nums: &[i64], target: &i64) -> bool {

    let mut res = false;
    // Space complexity O(n)
    let mut num_map: HashMap<i64, i64> = HashMap::new();

    // Time complexity O(n)
    for (i,num) in nums.iter().enumerate(){
        num_map.insert(*num,i as i64);

    }

    // Time complexity O(n). Acc O(2n)
    for (i,num) in nums.iter().enumerate() {
        let complement:i64 = target - num;

        //println!("{} {:?}",complement, Some(num_map.get(&complement)));

        if let Some(&index) = num_map.get(&complement){
            //println!("got here");
            if index != i as i64 && num != &complement {
                res = !vec![i as i64, index].is_empty();
            }
        }
    }
    res
}