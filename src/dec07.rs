use std::fs::read_to_string;

type Equation = (usize, Vec<usize>);
enum Ops {
    ADD,
    MULT, 
    CONCAT, 
}

/// correct answer is 5837374519342
pub fn part1() -> usize {
    let eqs = get_input();

    let mut sum = 0;

    for (lh, rh) in eqs {
        let results = calculate(rh[0], rh[1..].to_vec(), false, lh);
        if results.contains(&lh) {
            sum += lh;
        }
    }
    return sum;
}

/// correct answer is 492383931650959
pub fn part2() -> usize {
    let eqs = get_input();

    let mut sum = 0;

    for (lh, rh) in eqs {
        let results = calculate(rh[0], rh[1..].to_vec(), true, lh);
        if results.contains(&lh) {
            sum += lh;
        }
    }
    return sum;
}


fn calculate(current_result: usize, parts: Vec<usize>, concat: bool, goal: usize) -> Vec<usize> {
    if current_result > goal { return vec!() };
    if parts.len() == 0 { return vec!(current_result) };

    let mut add_result = calculate(current_result+parts[0], parts[1..].to_vec(), concat, goal);
    let mut mult_result = calculate(current_result*parts[0], parts[1..].to_vec(), concat, goal);
    add_result.append(&mut mult_result);
    if concat {
        let mut value_as_string: String = current_result.to_string();
        value_as_string.push_str(&parts[0].to_string());
        let mut concat_result = calculate(value_as_string.parse().unwrap(), parts[1..].to_vec(), concat, goal);
        add_result.append(&mut concat_result);
    }
    add_result
}




fn get_input() -> Vec<Equation> {
    let mut input = vec!();
    for line in read_to_string("src/in/dec07.in").unwrap().lines() {
        let split: Vec<&str> = line.split(":").collect();
        let lh: usize = split[0].parse().unwrap();
        let rh: Vec<usize> = split[1].split_whitespace().map(|n| n.parse().unwrap()).collect();
        input.push((lh, rh));
    }
    input
}