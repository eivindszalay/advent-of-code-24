use std::{collections::HashMap, fs::read_to_string};


/// correct answer is 197357
pub fn part1() -> usize {
    read_parse_and_calculate(25)
}

/// correct answer is 234568186890978
pub fn part2() -> usize {
    read_parse_and_calculate(75)
}

fn read_parse_and_calculate(blinks: u8) -> usize {
    let input: Vec<u128> = read_to_string("src/in/dec11.in")
        .unwrap()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();

    let mut calculated: HashMap<(u128, u8), usize> = HashMap::new();
    let mut sum = 0;
    for n in input {
        sum += calculate(n, blinks, &mut calculated);
    }
    sum
}



fn calculate(n: u128, depth: u8, calculated: &mut HashMap<(u128, u8), usize>) -> usize {
    if let Some(res) = calculated.get(&(n, depth)) {
       return *res;
    }
    let sum;
    if depth == 0 {
        sum = 1;
    } else if n == 0 {
        sum = calculate(1, depth-1, calculated);
    } else if n == 1 {
        sum = calculate(2024, depth-1, calculated)
    } else {
        let s = n.to_string();
        if s.len()%2==0 {
            let s1: u128 = s[..s.len()/2].parse().unwrap();
            let s2: u128 = s[s.len()/2..].parse().unwrap();
            sum = calculate(s1, depth-1, calculated)+calculate(s2, depth-1, calculated);
        } else {
            sum = calculate(n*2024, depth-1, calculated);
        }
    }
    calculated.insert((n, depth), sum);
    sum
}