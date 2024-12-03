use std::fs::read_to_string;
use regex::Regex;

pub fn dec03a() {
    let memory = read_to_string("src/in/dec03.in").unwrap();

    let outer_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let inner_re = Regex::new(r"\d{1,3}").unwrap();
    let mut sum = 0;

    let matches = outer_re.find_iter(&memory);

    for m in matches {
        let factors = inner_re.find_iter(m.into());
        let i_factors: Vec<i32> = factors.map(|factor| factor.as_str().parse::<i32>().unwrap()).collect();
        
        sum += i_factors[0]*i_factors[1];
    }

    println!("{}", sum);
}

pub fn dec03b() {
    let memory = read_to_string("src/in/dec03.in").unwrap();
    let outer_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let inner_re = Regex::new(r"\d{1,3}").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    let matches = outer_re.find_iter(&memory);    

    for m in matches {
        if m.as_str()=="do()" {
            enabled=true; 
            continue;
        }
        if m.as_str()=="don't()" {
            enabled=false; 
            continue;
        }
        if !enabled {
            continue
        };
        let factors = inner_re.find_iter(m.into());
        let i_factors: Vec<i32> = factors.map(|factor| factor.as_str().parse::<i32>().unwrap()).collect();
        
        sum += i_factors[0]*i_factors[1];
    }

    println!("{}", sum);
}