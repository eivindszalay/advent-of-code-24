use std::{collections::HashSet, fs::read_to_string};

pub fn part1() -> usize {
    let map: Vec<Vec<char>> = read_to_string("src/in/dec12.in")
        .unwrap()
        .lines()
        .map(|l| l
            .chars()
            .collect()
        )
        .collect();
    let mut sum = 0;
    let mut visited_regions: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited_regions.contains(&(x, y)) {
                let (area, perimeter, visited) = expand((x,y), &map);
                sum += area*perimeter;
                visited_regions.extend(visited);
            }
        }
    }
    sum
}

fn expand(start: (usize, usize), map: &Vec<Vec<char>>) -> (usize, usize, HashSet<(usize, usize)>) {
    let mut stack = vec!(start);
    let mut area = 0;
    let mut perimeter = 0;
    let plant = map[start.1][start.0];
    let mut visited_points: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x,y)) = stack.pop() {
        if map[y][x] != plant {
            perimeter += 1;
            continue;
        }
        if visited_points.contains(&(x,y)) {
            continue;
        }
        visited_points.insert((x,y));
        area += 1;
        if let Some(res) = x.checked_sub(1) {
            stack.push((res, y))
        } else {
            perimeter += 1;
        }
        if x+1<map[0].len() {
            stack.push((x+1, y))
        } else {
            perimeter += 1;
        }
        if let Some(res) = y.checked_sub(1) {
            stack.push((x, res))
        } else {
            perimeter += 1;
        }
        if y+1<map.len() {
            stack.push((x, y+1))
        } else { 
            perimeter += 1
        }
    }
    (area, perimeter, visited_points)
}
pub fn part2() -> usize {
    let map: Vec<Vec<char>> = read_to_string("src/in/dec12.in")
        .unwrap()
        .lines()
        .map(|l| l
            .chars()
            .collect()
        )
        .collect();
    let mut sum = 0;
    let mut visited_regions: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited_regions.contains(&(x, y)) {
                let (area, corners, visited) = expand_2((x,y), &map);
                sum += area*corners;
                visited_regions.extend(visited);
            }
        }
    }
    sum
}

fn expand_2(start: (usize, usize), map: &Vec<Vec<char>>) -> (usize, usize, HashSet<(usize, usize)>) {
    let mut stack = vec!(start);
    let mut area = 0;
    let mut corners = 0;
    let plant = map[start.1][start.0];
    let mut visited_points: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x,y)) = stack.pop() {
        if map[y][x] != plant {
            continue;
        }
        if visited_points.contains(&(x,y)) {
            continue;
        }
        visited_points.insert((x,y));
        area += 1;


        let n = get_neighbor_plant((x, y), Direction::N, &map);
        let ne = get_neighbor_plant((x, y), Direction::NE, &map);
        let e = get_neighbor_plant((x, y), Direction::E, &map);
        let se = get_neighbor_plant((x, y), Direction::SE, &map);
        let s = get_neighbor_plant((x, y), Direction::S, &map);
        let sw = get_neighbor_plant((x, y), Direction::SW, &map);
        let w = get_neighbor_plant((x, y), Direction::W, &map);
        let nw = get_neighbor_plant((x, y), Direction::NW, &map);

        if n==Some(plant) {
            stack.push((x, y-1));
        }
        if e==Some(plant) {
            stack.push((x+1, y));
        }
        if s==Some(plant) {
            stack.push((x, y+1));
        }
        if w==Some(plant) {
            stack.push((x-1, y));
        }

        if n!=Some(plant) && e!=Some(plant) {
            corners += 1;
        }
        if e!=Some(plant) && s!=Some(plant) {
            corners += 1;
        }
        if s!=Some(plant) && w!=Some(plant) {
            corners += 1;
        }
        if w!=Some(plant) && n!=Some(plant) {
            corners += 1;
        }

        if n==Some(plant) && e==Some(plant) && ne!=Some(plant) {
            corners += 1;
        }
        if e==Some(plant) && s==Some(plant) && se!=Some(plant) {
            corners += 1;
        }
        if s==Some(plant) && w==Some(plant) && sw!=Some(plant) {
            corners += 1;
        }
        if w==Some(plant) && n==Some(plant) && nw!=Some(plant) {
            corners += 1;
        }
    }
    (area, corners, visited_points)
}

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

fn get_neighbor_plant((x, y): (usize, usize), dir: Direction, map: &Vec<Vec<char>>) -> Option<char> {
    match dir {
        Direction::N => {
            if let Some(r) = y.checked_sub(1) {
                return Some(map[r][x]);
            }
            return None;
        },
        Direction::NE => {
            if let Some(r) = y.checked_sub(1) {
                if x+1<map[0].len() {
                    return Some(map[r][x+1]);
                } 
            }
            return None;
        },
        Direction::E => {
            if x+1 < map[0].len() {
                return Some(map[y][x+1]);
            }
            return None;
        },
        Direction::SE => {
            if y+1 < map.len() {
                if x+1 < map[0].len() {
                    return Some(map[y+1][x+1]);
                } 
            }
            return None;
        },
        Direction::S => {
            if y+1 < map.len() {
                return Some(map[y+1][x]);
            }
            return None;
        },
        Direction::SW => {
            if y+1 < map.len() {
                if let Some(r) = x.checked_sub(1) {
                    return Some(map[y+1][r]);
                } 
            }
            return None;
        },
        Direction::W => {
            if let Some(r) = x.checked_sub(1) {
                return Some(map[y][r]);
            }
            return None;
        },
        Direction::NW => {
            if let Some(r1) = y.checked_sub(1) {
                if let Some(r2) = x.checked_sub(1) {
                    return Some(map[r1][r2]);
                } 
            }
            return None;
        },
    };
}