use std::fs::read_to_string;

pub fn part1() -> u32 {

    let mut sum = 0;

    for line in read_to_string("src/in/dec02.in").unwrap().lines() {
        let levels: Vec<i32> = line.split(" ").map(|level| level.parse().unwrap()).collect();

        if safe(&levels) {
            sum += 1;
        }

    }

    return sum;
}

pub fn part2() -> u32 {

    let mut sum = 0;

    for line in read_to_string("src/in/dec02.in").unwrap().lines() {
        let mut levels: Vec<i32> = line.split(" ").map(|level| level.parse().unwrap()).collect();

        if safe(&levels) {
            sum += 1;
        } else {
            let mut i = 0;
            let length = levels.len();
            while i < length {
                let removed = levels.remove(i);
                if safe(&levels) {
                    sum += 1;
                    i = length;
                }
                else {
                    levels.insert(i, removed);
                    i += 1;
                }
            }
        }
    }

    return sum;
}

fn safe(levels: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut i = 0;
    while i < levels.len() - 1 {
        let diff = levels[i+1] - levels[i];
        if levels[0] < levels[1] {
            safe = safe && 1 <= diff && diff <=3;
        } else {
            safe = safe && -3 <= diff && diff <=-1;
        }

        i += 1;
    }
    return safe;
}