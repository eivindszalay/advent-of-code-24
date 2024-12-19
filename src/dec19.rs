use std::{collections::HashMap, fs::read_to_string};

/// correct answer is 265
pub fn part1() -> usize {
    let i = read_to_string("src/in/dec19.in").unwrap();
    let input = i.replace("\r\n", "\n");
    let (t, p) = input.split_once("\n\n").unwrap();

    let towels: Vec<String> = t
        .split(", ").map(|towel| towel.to_string()).collect();

    let patterns: Vec<String> = p.lines().map(|pattern| pattern.to_string()).collect();
    let mut sum = 0;
    for pattern in patterns.iter() {
        if can_make_pattern(pattern, &towels) {
            sum += 1;
        }
    }
    sum
}

fn can_make_pattern(pattern: &String, towels: &Vec<String>) -> bool{
    if pattern.len() == 0 {
        return true;
    }

    for towel in towels.iter() {
        if pattern.len() >= towel.len() {
            if pattern[..towel.len()]==*towel {
                if can_make_pattern(&pattern[towel.len()..].to_string(), towels) {
                    return true;
                }
            }
        }
    }
    return false;
}

/// correct answer is 752461716635602
pub fn part2() -> usize {
    let i = read_to_string("src/in/dec19.in").unwrap();
    let input = i.replace("\r\n", "\n");
    let (t, p) = input.split_once("\n\n").unwrap();

    let towels: Vec<String> = t
        .split(", ").map(|towel| towel.to_string()).collect();

    let patterns: Vec<String> = p.lines().map(|pattern| pattern.to_string()).collect();
    let mut sum = 0;
    for pattern in patterns.iter() {
        let mut counted: HashMap<String, usize> = HashMap::new();
        sum += can_make_pattern_counted(pattern, &towels, &mut counted);
    }
    sum
}

fn can_make_pattern_counted(pattern: &String, towels: &Vec<String>, counted: &mut HashMap<String, usize>) -> usize{
    if let Some(count) = counted.get(pattern) {
        return *count;
    }
    if pattern.len() == 0 {
        return 1;
    }
    let mut sum = 0;
    for towel in towels.iter() {
        if pattern.len() >= towel.len() {
            if pattern[..towel.len()]==*towel {
                sum += can_make_pattern_counted(&pattern[towel.len()..].to_string(), towels,  counted);
            }
        }
    }
    counted.insert(pattern.clone(), sum);
    sum
}