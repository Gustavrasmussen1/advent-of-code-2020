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
    
    let filename = "./dec6_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let input:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let groups:Vec<Vec<String>> = get_groups(&input);
    //println!("{:?}", groups);

    let group_unique_answers: Vec<i32> = groups.iter().map(|g| count_unique_answers(g)).collect();
    let sum_of_unique = group_unique_answers.iter().sum::<i32>(); 
    // Part 1 
    println!("{}", sum_of_unique);

    let group_mutual_answers: Vec<i32> = groups.iter().map(|g| count_mutual_answers(g)).collect();
    // Part 2
    let sum_of_mutual = group_mutual_answers.iter().sum::<i32>();
    println!("{}", sum_of_mutual);

}

fn get_groups(i: &[String]) -> Vec<Vec<String>> {
    // A vector of all groups
    // where each group is a vector of Strings
    let mut groups:Vec<Vec<String>> = Vec::new();
    let mut curr_group:Vec<String> = Vec::new();
    
    for line in i {
        if line.is_empty() {
            let final_group = curr_group.clone();
            groups.push(final_group);
            curr_group.clear();            
        }
        else{
            curr_group.push(line.to_string());
        }
    }

    // Get last group
    let final_group = curr_group.clone();
    groups.push(final_group);
    curr_group.clear();   

    // Return groups
    groups
}

fn count_unique_answers(answers: &[String]) -> i32{
    
    let mut unique_answers:Vec<String> = Vec::new();

    for a in answers {
        for x in a.chars()
        {
            if !unique_answers.contains(&x.to_string()) { 
                unique_answers.push(x.to_string());
            }
        }
    }
    
    let result = unique_answers.len() as i32;
    result
}

fn count_mutual_answers(answers: &[String]) -> i32 {

    let mut unique_answers:Vec<String> = Vec::new();
    let mut mutual_answers:Vec<String> = Vec::new();

    for a in answers {
        for x in a.chars()
        {
            if !unique_answers.contains(&x.to_string()) { 
                unique_answers.push(x.to_string());
            }
        }
    }

    let mut boo: bool = true;
    for a in unique_answers {
        boo = true;

        for b in answers {
            if !split_string_to_vector(b).contains(&a){
                boo = false;
            }
        }

        if boo { mutual_answers.push(a)};
    }
    
    let result = mutual_answers.len() as i32;
    result
}

fn split_string_to_vector(s: &String) -> Vec<String> {

    let split:Vec<String> = s.chars().map(|c| c.to_string()).collect();
    split
}