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
    let filename = "./dec10_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut input:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let mut nums:Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut num_map = HashMap::new();


    let highest_jolt = nums.iter().max().unwrap().clone() + 3;
    // add 0 jolts and max jolts to nums
    nums.push(0);
    nums.push(highest_jolt);

    for (i,num) in nums.iter().enumerate(){
        num_map.insert(*num,i as i32);
    }

    let mut diff:Vec<i32> = vec![];
    //nums.push(*highest_jolt);


    for i in 0..highest_jolt+1 {
        if let Some(&n) = num_map.get(&i)
        {
            for j in vec![1,3]{
                let complement = j+i;
                if let Some(&index) = num_map.get(&complement) {
                    diff.push(complement - i);
                    break;
                }
            }
        }

    }
    //println!("{:?}", diff);
    println!("1 Diff: {:?}", diff.iter().filter(|&n| *n==1).count());
    println!("3 Diff {:?}", diff.iter().filter(|&n| *n==3).count());
    println!("Multiplied {}", diff.iter().filter(|&n| *n==1).count()*diff.iter().filter(|&n| *n==3).count());

    // Part 2 

    nums.sort(); // inplace sort
    println!("{:?}",nums);

    let mut x = 0;

    println!("{:?}",count_arrangements(&nums, &mut x));

} 


fn count_arrangements(nums: &[i32], count: &mut u128) -> u128{

    for i in 1..nums.len() -1 {
        // Check possible swaps
        // println!("Possible to swap {} to {:?}",nums[i],possible_swaps(&nums,i as i32));
        let valid_swaps = possible_swaps(&nums,i as i32);
        for swap in valid_swaps {
            let mut reduced_nums:Vec<i32> = nums.iter().filter(|n| *n >= &swap).map(|n| *n).collect::<Vec<i32>>();
            //println!("Swapped {} with {} to get {:?}",nums[i], swap, reduced_nums);
            *count +=1 ;
            count_arrangements(&reduced_nums, count);
        }
    }
    *count +1 
}

fn valid_arrangements(nums: &[i32]) -> bool{

    let valid = true;

    for i in 1..nums.len()-1{

        if nums[i] - nums[i-1] > 3{
            let valid = false;
        }
    }
    valid
}

fn possible_swaps(nums: &[i32],index: i32) -> Vec<i32>{
    
    let num = nums[index as usize];
    let mut res:Vec<i32> = vec![];

    if nums.contains(&(num+1)) {
        if !(num+1 - nums[index as usize - 1] > 3){
            res.push(num+1)
        }
        
    }
    if nums.contains(&(num+2)) {
        if !(num + 2 - nums[index as usize - 1] > 3){
            res.push(num+2)
        }
    }

    res
}

