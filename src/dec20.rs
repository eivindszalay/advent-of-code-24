use std::{collections::{HashMap, HashSet}, fs::read_to_string};

/// correct answer is 1360
pub fn part1() -> usize {
    get_solution(2)
}

/// correct answer is 1005476
pub fn part2() -> usize {
    get_solution(20)
}

fn get_solution(depth: usize) -> usize {
    let race_track: Vec<Vec<char>> = read_to_string("src/in/dec20.in")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    let (mut x, mut y): (usize, usize) = (0,0);
    for i in 0..race_track.len() {
        for j in 0..race_track[0].len() {
            if race_track[i][j] == 'S' {
                (x, y) = (j, i);
            }
        }
    }
    let mut current_distance = 0;
    while race_track[y][x] != 'E' {
        distances.insert((x, y), current_distance);
        current_distance += 1;
        if 0<x && race_track[y][x-1] != '#' && race_track[y][x-1] != 'S' && !distances.contains_key(&(x-1, y)) {
            x -= 1;
            continue;
        }
        if x<race_track[0].len()-1 && race_track[y][x+1] != '#' && race_track[y][x+1] != 'S' && !distances.contains_key(&(x+1, y)) {
            x += 1;
            continue;
        }
        if 0<y && race_track[y-1][x] != '#' && race_track[y-1][x] != 'S' && !distances.contains_key(&(x, y-1)) {
            y -= 1;
            continue;
        }
        if y<race_track.len()-1 && race_track[y+1][x] != '#' && race_track[y+1][x] != 'S' && !distances.contains_key(&(x, y+1)) {
            y += 1;
            continue;
        }
    }
    distances.insert((x, y), current_distance);

    let mut number_of_cheats = 0;
    for point in distances.keys() {
        number_of_cheats += get_cheats(&race_track, *point, depth, &distances, current_distance);
    }
    number_of_cheats
}

fn get_cheats(
    race_track: &Vec<Vec<char>>,
    start_point: (usize, usize),
    depth: usize,
    distances: &HashMap<(usize, usize), usize>,
    track_length: usize
) -> usize {
    let mut cheats_found: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    for y in 0..depth+1 {
        for x in 0..depth+1 {
                if let Some (y_subtracted) = start_point.1.checked_sub(y) {
                if let Some (x_subtracted) = start_point.0.checked_sub(x) {
                    if is_good_cheat(race_track, start_point, (x_subtracted, y_subtracted), depth, distances, track_length) {
                        cheats_found.insert(((start_point), (x_subtracted, y_subtracted)));
                    }
                }
                let x_added = start_point.0 + x;
                if x_added < race_track[0].len() {
                    if is_good_cheat(race_track, start_point, (x_added, y_subtracted), depth, distances, track_length) {
                        cheats_found.insert(((start_point), (x_added, y_subtracted)));

                    }
                }
                
            }
        }
        let y_added = start_point.1 + y;
        if y_added < race_track.len() {
            for x in 0..depth+1 {
                if let Some (x_subtracted) = start_point.0.checked_sub(x) {
                    if is_good_cheat(race_track, start_point, (x_subtracted, y_added), depth, distances, track_length) {
                        cheats_found.insert(((start_point), (x_subtracted, y_added)));
                    }
                }
                let x_added = start_point.0 + x;
                if x_added < race_track[0].len() {
                    if is_good_cheat(race_track, start_point, (x_added, y_added), depth, distances, track_length) {
                        cheats_found.insert(((start_point), (x_added, y_added)));
                    }
                }
            }
        }
    }
    cheats_found.len()
}

fn is_good_cheat(
    race_track: &Vec<Vec<char>>,
    start_point: (usize, usize),
    (x, y): (usize, usize),
    depth: usize,
    distances: &HashMap<(usize, usize), usize>,
    track_length: usize
) -> bool {
    let is_free_tile = race_track[y][x] != '#';
    if !is_free_tile { return false; }

    let cheat_length = start_point.0.abs_diff(x) + start_point.1.abs_diff(y);
    if depth < cheat_length { return false; }
    
    let start_to_cheat_length = distances.get(&start_point).unwrap();
    let cheat_to_end_length = track_length - distances.get(&(x, y)).unwrap();
    let total_length = start_to_cheat_length + cheat_length + cheat_to_end_length;
    
    return total_length <= track_length - 100;
}