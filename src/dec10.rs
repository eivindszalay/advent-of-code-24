use std::{collections::{HashMap, HashSet}, fs::read_to_string};


/// correct answer is 430
pub fn part1() -> usize {
    let input: Vec<String> = read_to_string("src/in/dec10.in")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    let height = input.len();
    let length = input[0].len();
    let mut trail_head_scores: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            if input[y].chars().nth(x).unwrap() == '0' {
                let trail_heads = walk_trail((x, y), &input);
                for head in trail_heads {
                    let score = trail_head_scores.get(&head);
                    if let Some(s) = score {
                        trail_head_scores.insert(head, s+1);
                    } else {
                        trail_head_scores.insert(head, 1);
                    }
                }
            }
        }
    }

    trail_head_scores.values().sum()

}

fn walk_trail((x, y): (usize, usize), map: &Vec<String>) -> HashSet<(usize, usize)> {
    let height = map.len();
    let length = map[0].len();
    let current_alt = map[y].chars().nth(x).unwrap().to_string().parse::<usize>().unwrap();
    let mut trail_heads: HashSet<(usize, usize)> = HashSet::new();
    if current_alt==9 {
        trail_heads.insert((x, y));
        return trail_heads;
    }
    if x+1<length {
        let new_alt = map[y].chars().nth(x+1).unwrap().to_string().parse::<usize>().unwrap();
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x+1, y), &map));
        }
    }
    if y+1<height {
        let new_alt = map[y+1].chars().nth(x).unwrap().to_string().parse::<usize>().unwrap();
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x, y+1), &map));
        }
    }
    if let Some(decrement_x) = x.checked_sub(1) {
        let new_alt = map[y].chars().nth(decrement_x).unwrap().to_string().parse::<usize>().unwrap();
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((decrement_x, y), &map));
        }
    }
    if let Some(decrement_y) = y.checked_sub(1) {
        let new_alt = map[decrement_y].chars().nth(x).unwrap().to_string().parse::<usize>().unwrap();
        if current_alt+1 == new_alt {
            trail_heads.extend(walk_trail((x, decrement_y), &map));
        }
    }

    trail_heads
}   


pub fn part2() -> usize {0}