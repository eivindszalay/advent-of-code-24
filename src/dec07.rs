use std::fs::read_to_string;

/// correct answer is 5837374519342
pub fn part1() -> usize {
    return get_sum(false);
}

/// correct answer is 492383931650959
pub fn part2() -> usize {
    return get_sum(true);
}

fn get_sum(concat: bool) -> usize {
    let mut input = vec!();
    for line in read_to_string("src/in/dec07.in").unwrap().lines() {
        let split: Vec<&str> = line.split(":").collect();
        let lh: usize = split[0].parse().unwrap();
        let rh: Vec<usize> = split[1].split_whitespace().map(|n| n.parse().unwrap()).collect();
        input.push((lh, rh));
    }

    let mut sum = 0;
    for (lh, rh) in input {
        let possible = calculate_possibility(rh[0], rh[1..].to_vec(), concat, lh);
        if possible {
            sum += lh;
        }
    }
    sum
}



fn calculate_possibility(current_result: usize, parts: Vec<usize>, concat: bool, goal: usize) -> bool {
    if current_result > goal { return false };
    if parts.len() == 0 { return current_result==goal };

    let add_result = calculate_possibility(current_result+parts[0], parts[1..].to_vec(), concat, goal);
    if add_result { return true };
    let mult_result = calculate_possibility(current_result*parts[0], parts[1..].to_vec(), concat, goal);
    if mult_result { return true };
    let mut concat_result = false;
    if concat {
        let mut value_as_string: String = current_result.to_string();
        value_as_string.push_str(&parts[0].to_string());
        concat_result = calculate_possibility(value_as_string.parse().unwrap(), parts[1..].to_vec(), concat, goal);
    }
    return concat_result;
}