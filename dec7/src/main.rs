use std::fs::File;
use std::io::{BufRead, BufReader};
//use itertools::Itertools;
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

    let filename = "./dec7_input.txt";
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let input:Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    
    //println!("{:?}",parse_rules(&input));
    let mut x = 0;

    let mut rules = parse_rules(&input);
    let mut valid_outer_bags:Vec<String> = Vec::new();


    for (color, bag) in &rules {
        //println!("{:?}", bag.can_contain);
        if bag.can_carry("shinygold") > 0 {
            
            x += 1;
            
            if !valid_outer_bags.contains(&color) {valid_outer_bags.push(color.to_string());}
            continue;
        }
        else {
                for (c,b) in &bag.can_contain {

                    if &rules[c].can_carry("shinygold") > &0 {
                        if !valid_outer_bags.contains(&color) {valid_outer_bags.push(color.to_string());}
                        x += 1;
                        continue; 
                    }

                    else {
                        for (cc,bb) in &rules[c].can_contain {
                            
                            if &&rules[cc].can_carry("shinygold") > &&0 {
                                if !valid_outer_bags.contains(&&color) {valid_outer_bags.push(color.to_string());}
                                x += 1;
                                continue; 
                            }

                            else {
                                for (ccc,bbb) in &rules[cc].can_contain {
                                    
                                    if &&&rules[ccc].can_carry("shinygold") > &&&0 {
                                        if !valid_outer_bags.contains(&&&color) {valid_outer_bags.push(color.to_string());}
                                        x += 1;
                                        continue; 
                                    }

                                    else {
                                        for (cccc,bbbb) in &rules[ccc].can_contain {
                                            
                                            if &&&&rules[cccc].can_carry("shinygold") > &&&&0 {
                                                if !valid_outer_bags.contains(&&&&color) {valid_outer_bags.push(color.to_string());}
                                                x += 1;
                                                continue; 
                                            }

                                            else {
                                                for (ccccc,bbbbb) in &rules[cccc].can_contain {
                                                    
                                                    if &&&&&rules[ccccc].can_carry("shinygold") > &&&&&0 {
                                                        if !valid_outer_bags.contains(&&&&&color) {valid_outer_bags.push(color.to_string());}
                                                        x += 1;
                                                        continue; 
                                                    }

                                                    else {
                                                        for (cccccc,bbbbbb) in &rules[ccccc].can_contain {
                                                            
                                                            if &&&&&&rules[cccccc].can_carry("shinygold") > &&&&&&0 {
                                                                if !valid_outer_bags.contains(&&&&&&color) {valid_outer_bags.push(color.to_string());}
                                                                x += 1;
                                                                continue; 
                                                            }

                                                            else {
                                                                for (ccccccc,bbbbbbb) in &rules[cccccc].can_contain {
                                                                    
                                                                    if &&&&&&&rules[ccccccc].can_carry("shinygold") > &&&&&&&0 {
                                                                        if !valid_outer_bags.contains(&&&&&&&color) {valid_outer_bags.push(color.to_string());}
                                                                        x += 1;
                                                                        continue; 
                                                                    }

                                                                    else {
                                                                        for (cccccccc,bbbbbbbb) in &rules[ccccccc].can_contain {
                                                                            
                                                                            if &&&&&&&&rules[cccccccc].can_carry("shinygold") > &&&&&&&&0 {
                                                                                if !valid_outer_bags.contains(&&&&&&&&color) {valid_outer_bags.push(color.to_string());}
                                                                                x += 1;
                                                                                continue; 
                                                                            }

                                                                            else {
                                                                                for (ccccccccc,bbbbbbbbb) in &rules[cccccccc].can_contain {
                                                                                    
                                                                                    if &&&&&&&&&rules[ccccccccc].can_carry("shinygold") > &&&&&&&&&0 {
                                                                                        if !valid_outer_bags.contains(&&&&&&&&&color) {valid_outer_bags.push(color.to_string());}
                                                                                        x += 1;
                                                                                        continue; 
                                                                                    }
                                                                                }
                                                                            
                                                                        }
                                                                        }
                                                                    
                                                                }

                                                                }
                                                            
                                                        }
                                                        }
                                                    
                                                }
                                                }
                                            
                                        }

                                        }
                                    
                                }
                                }
                            
                        }
                        }
                    
                }
                }
            
        }

        //println!("{} {:?}", color, bag.can_carry("shinygold"));
    }
    
    println!("{:?}", valid_outer_bags.len());

    //println!("{:?}", &rules["darkolive"].can_contain);

    println!("{:?}", &rules["shinygold"].sum_of_inner_bags);
    
    let mut cost = 0;

    for (c,b) in &rules["shinygold"].can_contain {

        let mut sum2 = 0;
        for (c2,b2) in &rules[c].can_contain {

            let mut sum3 = 0;
            for (c3,b3) in &rules[c2].can_contain {
                
                let mut sum4 = 0;
                for (c4,b4) in &rules[c3].can_contain {

                    let mut sum5 = 0;
                    for (c5,b5) in &rules[c4].can_contain {

                        let mut sum6 = 0;
                        for (c6,b6) in &rules[c5].can_contain {

                            let mut sum7 = 0;
                            for (c7,b7) in &rules[c6].can_contain {
                                let mut sum8 = 0;
                                for (c8, b8) in &rules[c7].can_contain {
                                    sum8 += b8
                                }

                                sum7 += b7*sum8 +b7;
                            }

                            sum6 += b6*sum7+b6;
                        }

                        sum5 += b5*sum6 + b5;
                    }

                    sum4 += b4*sum5 + b4;
                }

                sum3 += b3*sum4 + b3;
                
            }
            sum2 += b2*sum3 + b2;
        }

        cost += b*sum2 + b;
    }

    println!("{}", cost);
}


fn parse_rules(_input: &[String]) -> HashMap<String, Bag> {

    let mut bags:HashMap<String,Bag> = HashMap::new();
    
    //println!("{:?}", my_bag.can_carry("Gold"));

    for (color, b) in &bags {
        //println!("Outer Bag {} contains {:?}", color, b.can_contain)
    }
    //println!("{:?}", get_inner_bags(&_input[0]));

    for line in _input {
        //println!("{}", line);
        //println!("FINAL OUTPUT: {} {:?}", get_outer_bag(&line), get_inner_bags(&line));
        let color = get_outer_bag(&line);
        let mut new_bag = Bag::new(&color);

        for (k,v) in get_inner_bags(&line)
        {
            new_bag.can_contain(&k,v);
        }

        //let copy_bag = new_bag.clone();
        bags.insert(color, new_bag);
    }

    bags
}

fn get_outer_bag(rule: &String) -> String {
    
    let words: Vec<&str> = rule
    .split_whitespace()
    .collect();
    
    let outer_bag = words[0].to_string() + &words[1].to_string();

    outer_bag
}

fn get_inner_bags(rule: &String) -> HashMap<String, i32> {

    let mut inner_bags:HashMap<String,i32> = HashMap::new();

    let words: Vec<&str> = rule
    .split("contain")
    .collect();


    let rhs: Vec<&str> = words[1]
    .split_whitespace()
    .collect();
   
    //println!("{:?}", rhs);

    let mut quant = 0;
    let mut a = String::from("");
    let mut b = String::from("");
    let mut final_color = String::from("");

    let cut_words = vec!["bag,", "bags.","bags,", "bag."]; // handle 'no' case by itself

    for word in rhs{

        //println!("{}", word);
        if word == "no"{
            //println!("NO CONDITION!");
            break;
        }
        else if cut_words.contains(&word) {
            //println!("CUT WORD CONDITON!");
            final_color = a.clone();
            final_color += &b;

            //println!("DEBUG {} {}", final_color, quant);
            inner_bags.insert(final_color, quant);
            
            a.clear();
            b.clear();
            quant = 0;
        }
        else if word.chars().all(char::is_numeric) {
            //println!("NUMERIC!");
            quant = word.parse::<i32>().unwrap();
        }
        else if !cut_words.contains(&word){
            if a.is_empty() {
                //println!("CHANGIN A");
                a = word.to_string();
            }
            else {
                //println!("CHANGING B");
                b = word.to_string();
            }
        }
        
    }

    inner_bags
}


struct Bag {
    color: String,
    can_contain: HashMap<String,i32>, // Color of bag it can contain and how many
    sum_of_inner_bags: i32,
}

impl Bag {
    fn new(_color: &str) -> Bag {
        Bag {
            color:_color.to_string(),
            can_contain: HashMap::new(),
            sum_of_inner_bags: 0,
        }
    }

    fn can_contain(&mut self, _color: &str, _size: i32) {
        self.can_contain.insert(_color.to_string(), _size);
        self.sum_of_inner_bags += _size;
    }

    fn can_carry(&self, _color: &str) -> i32{
        // implement -> i32 how many of String the Bag can contain
        let mut number = 0;

        for (c, n) in &self.can_contain {
            
            if c == _color {
                number = self.can_contain[c];
                //println!("{} {}",c, number);
            }
        }
        number
    }
}