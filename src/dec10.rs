use std::{collections::{HashMap, HashSet}, fs::read_to_string};


/// correct answer is 430
pub fn part1() -> usize {
    let input: Vec<Vec<usize>> = read_to_string("src/in/dec10.in")
        .unwrap()
        .lines()
        .map(|string| string
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect()
        )
        .collect();
    let height = input.len();
    let length = input[0].len();
    let mut trail_head_scores: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            if input[y][x] == 0 {
                let trail_tops = walk_trail((x, y), &input);
                for top in trail_tops {
                    let score = trail_head_scores.get(&top);
                    if let Some(s) = score {
                        trail_head_scores.insert(top, s+1);
                    } else {
                        trail_head_scores.insert(top, 1);
                    }
                }
            }
        }
    }

    trail_head_scores.values().sum()

}

fn walk_trail((x, y): (usize, usize), map: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let height = map.len();
    let length = map[0].len();
    let current_alt = map[y][x];
    let mut trail_heads: HashSet<(usize, usize)> = HashSet::new();
    if current_alt==9 {
        trail_heads.insert((x, y));
        return trail_heads;
    }
    if x+1<length {
        let new_alt = map[y][x+1];
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x+1, y), &map));
        }
    }
    if y+1<height {
        let new_alt = map[y+1][x];
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x, y+1), &map));
        }
    }
    if let Some(decrement_x) = x.checked_sub(1) {
        let new_alt = map[y][decrement_x];
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((decrement_x, y), &map));
        }
    }
    if let Some(decrement_y) = y.checked_sub(1) {
        let new_alt = map[decrement_y][x];
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x, decrement_y), &map));
        }
    }

    trail_heads
}   

/// correct answer is 928
pub fn part2() -> usize {
    let input: Vec<Vec<usize>> = read_to_string("src/in/dec10.in")
        .unwrap()
        .lines()
        .map(|string| string
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect()
        )
        .collect();
    let height = input.len();
    let length = input[0].len();
    let mut trail_head_scores: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            if input[y][x] == 0 {
                let score = walk_trail_2((x, y), &input);
                trail_head_scores.insert((x, y), score);
            }
        }
    }

    trail_head_scores.values().sum()
}


fn walk_trail_2((x, y): (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let height = map.len();
    let length = map[0].len();
    let current_alt = map[y][x];
    let mut sum=0;
    if current_alt==9 {
        return 1;
    }
    if x+1<length {
        let new_alt = map[y][x+1];
        if current_alt+1 == new_alt {
            sum += walk_trail_2((x+1, y), &map);
        }
    }
    if y+1<height {
        let new_alt = map[y+1][x];
        if current_alt+1 == new_alt {
            sum += walk_trail_2((x, y+1), &map);
        }
    }
    if let Some(decrement_x) = x.checked_sub(1) {
        let new_alt = map[y][decrement_x];
        if current_alt+1 == new_alt {
            sum += walk_trail_2((decrement_x, y), &map);
        }
    }
    if let Some(decrement_y) = y.checked_sub(1) {
        let new_alt = map[decrement_y][x];
        if current_alt+1 == new_alt {
            sum += walk_trail_2((x, decrement_y), &map);
        }
    }

    sum
}   