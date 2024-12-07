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
        let results = calculate(rh[0], rh[1..].to_vec(), false);
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
        let results = calculate(rh[0], rh[1..].to_vec(), true);
        if results.contains(&lh) {
            sum += lh;
        }
    }
    return sum;
}


fn calculate(value: usize, parts: Vec<usize>, concat: bool) -> Vec<usize> {
    if parts.len() == 0 { return vec!(value) };

    let mut add_result = calculate(value+parts[0], parts[1..].to_vec(), concat);
    let mut mult_result = calculate(value*parts[0], parts[1..].to_vec(), concat);
    add_result.append(&mut mult_result);
    if concat {
        let mut foo: String = value.to_string();
        foo.push_str(&parts[0].to_string());
        let mut concat_result = calculate(foo.parse().unwrap(), parts[1..].to_vec(), concat);
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