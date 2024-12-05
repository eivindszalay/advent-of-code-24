use std::{cmp::Ordering, fs::read_to_string};

type Rule = (usize, usize);

/// correct answer is 5747
pub fn dec05a() {
    let (rules, updates) = get_input();
    let mut sum = 0;

    for update in updates {
        let correct = is_correct(&update, &rules); 
        if correct {
            sum += update[update.len()/2];
        }
    }

    println!("{}", sum);
}

/// correct answer is 5502
pub fn dec05b() {
    let (rules, updates) = get_input();
    let mut sum = 0;
    for update in updates {
        let correct = is_correct(&update, &rules); 
        if !correct {
            sum += fix_update(update, &rules);
        }
    }

    println!("{}", sum);
}

fn fix_update(update: Vec<usize>, rules: &Vec<Rule>) -> usize {
    let mut sorted = update.clone();
    sorted.sort_by(|p1, p2| compare(*p1, *p2, rules));
    sorted[sorted.len()/2]
}

fn compare(p1: usize, p2: usize, rules: &Vec<Rule>) -> Ordering {
    if rules.contains(&(p1, p2)) {
        return Ordering::Less;
    } else if rules.contains(&(p2, p1)) {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

fn is_correct(update: &Vec<usize>, rules: &Vec<Rule>) -> bool {
    let mut correct = true; 
    for i in 0..update.len() {
        for j in i..update.len() {
            correct = correct && !rules.contains(&(update[j],update[i]))
        }
    }
    correct
}

fn get_input() -> (Vec<Rule>, Vec<Vec<usize>>) {
    let input = read_to_string("src/in/dec05.in").unwrap();
    let rules: Vec<Rule> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|rule| parse_as_rule(rule))
        .collect();

    let updates: Vec<Vec<usize>> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| parse_as_update(line))
        .collect();
    (rules, updates)
}

fn parse_as_rule(rule: &str) -> Rule {
    let pages: Vec<&str> = rule.split("|").collect();
    let left = pages[0].parse::<usize>().unwrap();
    let right = pages[1].parse::<usize>().unwrap();
    (left, right)
}

fn parse_as_update(update: &str) -> Vec<usize> {
    let pages: Vec<&str> = update.split(",").collect();
    pages
        .iter()
        .map(|page| page.parse::<usize>().unwrap())
        .collect()
}