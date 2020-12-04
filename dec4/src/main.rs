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

    let filename = "./dec4_input.txt";

    // Part 1
    count_valid_passports(filename,"1");
    count_valid_passports(filename, "2");

    // Part 2

}

fn count_valid_passports(s: &str, typ: &str){

    let filename = s;
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);

    // Populate Vector with passports
    let mut passports:Vec<String> = Vec::new();
    let mut pp = String::from("");

    for line in reader.lines() {
        let curr_line = line.unwrap();
        //println!("{}",curr_line);

        if !curr_line.is_empty() {
            //let c_clone = curr_line.clone();
            pp += &(" ".to_string() + &curr_line);    
        }
        else{
            let compl_pp = pp.clone();
            passports.push(compl_pp);
            pp.clear();
        }
        
    }
    // Add last passport - Where the empty line condition fails in above!
    let compl_pp = pp.clone();
    passports.push(compl_pp);

    // Count valid passports
    // valid_passport returns bool, so i is bool in filter() and only counted if true
    let valid_passports = passports.iter().map(|p| valid_passport(&p,typ)).filter(|&i| i).count();
    println!("{}", valid_passports);

    let valid_passports2 = passports.iter().map(|p| valid_passport(&p,typ)).filter(|&i| i).count();

    println!("{}", valid_passports2);
}

fn valid_passport(pp: &str, typ: &str) -> bool {

    let criteria = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    let x:Vec<_> = pp.split(" ").collect::<Vec<_>>();
    //x = x.iter().map
    let fields:Vec<_> = x.iter().map(|z| z.split(":").collect::<Vec<_>>()[0]).collect::<Vec<_>>();

    let collect = x.iter().map(|z| split_to_tuple(z)).collect::<Vec<_>>();
    let ppHash: HashMap<String,String> = collect.iter().cloned().collect();
    //println!("{:?}",ppHash);

    match typ {
        "1" => {
            let valid: bool = criteria.iter().all(|c| fields.contains(c));
            valid
        },
        "2" => {
            let valid_fields: bool = criteria.iter().all(|c| fields.contains(c));
            if !valid_fields {
                false // no need to check further if not all required fields are there!
            }
            else {
                    let mut a:Vec<_> = Vec::new();
                    for (k,v) in ppHash.iter() {
                        //println!("{} {} {}", k ,v, valid_value(k,v));
                        a.push(valid_value(k,v));

                        //println!("{} {} {}", k ,v, valid_value(k,v));

                    }
                    //println!("{:?}", !a.contains(&false));
                    !a.contains(&false)
            }
        },
        _ => false
    }
    


} 

fn valid_value(_field: &str, _value: &str) -> bool {
    // use match here

    //let _field = String::from(field);
    //let _value = String::from(value);
    
    // Helpers
    // 1. First character
    //let first_char = &_value[0];

    // 2. Last two characters

    match _field {
        "byr" => {
            1920 <= (_value.parse::<usize>().unwrap()) && (_value.parse::<usize>().unwrap())  <= 2002
        },
        "iyr" => {
            2010 <= (_value.parse::<usize>().unwrap()) && (_value.parse::<usize>().unwrap())  <= 2020
        },
        "eyr" => {
            2020 <= (_value.parse::<usize>().unwrap()) && (_value.parse::<usize>().unwrap())  <= 2030
        },
        "hgt" => {
            match _value.substring(_value.chars().count()-2,_value.chars().count()) {
                "cm" => {
                    150 <= (_value.substring(0,_value.chars().count()-2).parse::<usize>().unwrap()) && (_value.substring(0,_value.chars().count()-2).parse::<usize>().unwrap()) <= 193
                },
                "in" => {
                    59 <= (_value.substring(0,_value.chars().count()-2).parse::<usize>().unwrap()) && (_value.substring(0,_value.chars().count()-2).parse::<usize>().unwrap()) <= 76
                },
                _ => false
            }
        },
        "hcl" => {
            match _value.substring(0,1) {
                "#" => _value.chars().count() == 7, // Maybe check here if count char or bit?
                _ => false
            }
        },

        "ecl" => {
            match _value {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        },
        "pid" => {
            (_value.to_string().len() == 9) && (_value.chars().all(char::is_numeric))
        },
        "" => {true},
        _ => true
    }
}

fn split_to_tuple(s: &str) -> (String,String){

    // if empty return two empty strings
    if s.is_empty() {
        let result = ("".to_string(),"".to_string());
        result
    }
    else
    {
        let split:Vec<_> = s.split(":").collect();
        let result = (split[0].to_string(),split[1].to_string());
        result
    }

}
