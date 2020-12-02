use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "./dec2_input.txt";
    let result = count_valid_pws(filename);

    println!("{}",result);
}



fn count_valid_pws(s: &str) -> i32{
    let filename = s;
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    //let mut v: Vec<str> = vec![];

    let mut x = 0;
    for line in reader.lines() {
        
        //v.insert(line.unwrap()); //.parse().expect("unable to parse");
        //v.push(a);
        let s = line.unwrap();
        let chunks: Vec<_> = s.split_whitespace().collect();
        
        let policy = chunks[0];
        let letter = chunks[1];
        let pw = chunks[2];
        

        //println!("{}{}{}",policy,letter,pw);
        if check_if_valid(policy,letter,pw)
        {
            //println!("is Valid!");
            x = x + 1;
        }

    }

    // Return the Vec<i32>
    x

}

fn check_if_valid(_policy: &str, _letter: &str, _pw: &str) -> bool{
    
    let policy_rules:Vec<_> = _policy.split('-').collect();
    let _policy_min: usize = policy_rules[0].parse().unwrap();
    let _policy_max: usize = policy_rules[1].parse().unwrap();

    let t:Vec<_> = _letter.split(':').collect();
    let s = _pw.to_string();
    let n = s.matches(t[0]).count();

    if _policy_min <= n && n <= _policy_max { true }
    else { false } 
}
