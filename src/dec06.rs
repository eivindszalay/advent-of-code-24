use std::{collections::HashSet, fs::read_to_string, thread};

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    UP ,
    RIGHT,
    DOWN,
    LEFT
}

/// correct answer is 4789
pub fn part1() -> usize {
    let input = read_to_string("src/in/dec06.in").unwrap();
    let map: Vec<&str> = input
        .lines()
        .collect();
    let (mut x, mut y) = get_starting_pos(&map);

    let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();

    visited_pos.insert((x, y));

    let mut dir = Direction::UP;

    loop {
        let new_pos_result = match dir  {
            Direction::UP => move_up((x, y)),
            Direction::RIGHT => move_right((x, y), map[0].len()),
            Direction::DOWN => move_down((x, y), map.len()),
            Direction::LEFT => move_left((x, y)),
        };
        if let Ok((new_x, new_y)) = new_pos_result {
            if map[new_y].chars().nth(new_x).unwrap()=='#' {
                dir = match dir {
                    Direction::UP => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                }
            } else {
                x = new_x;
                y = new_y;
                visited_pos.insert((new_x, new_y));
            }
        } else {
            return visited_pos.len().try_into().unwrap();
        }
    }
}

/// correct answer is 1304
pub fn part2() -> usize {
    let input = read_to_string("src/in/dec06.in").unwrap();
    let map: Vec<&str> = input
        .lines()
        .collect();
    let mut pos = get_starting_pos(&map);

    
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    path.insert(pos);

    let mut dir = Direction::UP;

    loop {
        let new_pos_result = match dir  {
            Direction::UP => move_up(pos),
            Direction::RIGHT => move_right(pos, map[0].len()),
            Direction::DOWN => move_down(pos, map.len()),
            Direction::LEFT => move_left(pos),
        };
        if let Ok((new_x, new_y)) = new_pos_result {
            if map[new_y].chars().nth(new_x).unwrap()=='#' {
                dir = match dir {
                    Direction::UP => Direction::RIGHT,
                    Direction::RIGHT => Direction::DOWN,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                }
            } else {
                pos = (new_x, new_y);
                path.insert((new_x, new_y));
            }
        } else {
            break;
        }
    }
    
    let pos_1 = get_starting_pos(&map);
    let pos_2 = pos_1.clone();
    
    


    let path_array: Vec<(usize, usize)> = path.into_iter().collect();

    // for (visited_x, visited_y) in path {
    //     if (visited_x, visited_y) != pos {
    //         if has_loop(pos, &map, (visited_x,visited_y)) {
    //             sum += 1;
    //         }
    //     }
    // }

    let mid = path_array.len() / 2;
    let (first_half, second_half) = path_array.split_at(mid);

    let first_half = first_half.to_vec();
    let second_half = second_half.to_vec();

    let handle1 = thread::spawn(move || {
        let input_1 = read_to_string("src/in/dec06.in").unwrap();
        let map_1: Vec<&str> = input_1
            .lines()
            .collect();
        let mut sum = 0;
        for (visited_x, visited_y) in first_half {
            if (visited_x, visited_y) != pos_1 {
                if has_loop(pos_1, &map_1, (visited_x,visited_y)) {
                    sum += 1;
                }
            }
        }
        return sum;
    });

    let handle2 = thread::spawn(move || {
        let input_2 = read_to_string("src/in/dec06.in").unwrap();
        let map_2: Vec<&str> = input_2
            .lines()
            .collect();
        let mut sum = 0;
        for (visited_x, visited_y) in second_half {
            if (visited_x, visited_y) != pos_2 {
                if has_loop(pos_2, &map_2, (visited_x,visited_y)) {
                    sum += 1;
                }
            }
        }
        return sum;
    });
    let sum_1 = handle1.join().unwrap();
    let sum_2 = handle2.join().unwrap();

    return sum_1+sum_2;
}



fn has_loop((mut x, mut y): (usize, usize), map: &Vec<&str>, obstacle_pos: (usize, usize)) -> bool {
    let mut dir = &Direction::UP;
    let mut visited_pos: HashSet<(usize, usize, &Direction)> = HashSet::new();
    visited_pos.insert((x, y, &Direction::UP));
    loop {
        let new_pos_result = match dir  {
            Direction::UP => move_up((x, y)),
            Direction::RIGHT => move_right((x, y), map[0].len()),
            Direction::DOWN => move_down((x, y), map.len()),
            Direction::LEFT => move_left((x, y)),
        };
        if let Ok((new_x, new_y)) = new_pos_result {
            if map[new_y].chars().nth(new_x).unwrap()=='#' || (new_x, new_y)==obstacle_pos {
                
                dir = match dir {
                    Direction::UP => &Direction::RIGHT,
                    Direction::RIGHT => &Direction::DOWN,
                    Direction::DOWN => &Direction::LEFT,
                    Direction::LEFT => &Direction::UP,
                };
            } else {
                x = new_x;
                y = new_y;
                if visited_pos.contains(&(x, y, dir)) {
                    return true;
                } else {
                    visited_pos.insert((x, y, dir));
                }
            }
        } else {
            return false;
        }
    }
}


fn move_up((x, y): (usize, usize)) -> Result<(usize, usize), ()> {
    if let Some(new_y) = y.checked_sub(1) {
        return Ok((x, new_y));
    }
    Err(())
}
fn move_right((x, y): (usize, usize), width: usize) -> Result<(usize, usize), ()> {
    if x+1 >= width {
        return Err(())
    }
    return Ok((x+1, y));
}
fn move_down((x, y): (usize, usize), height: usize) -> Result<(usize, usize), ()> {
    if y+1 >= height {
        return Err(())
    }
    return Ok((x, y+1));
}
fn move_left((x, y): (usize, usize)) -> Result<(usize, usize), ()> {
    if let Some(new_x) = x.checked_sub(1) {
        return Ok((new_x, y));
    }
    Err(())
}

fn get_starting_pos(map: &Vec<&str>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y].chars().nth(x).unwrap()=='^' {
                return (x, y);
            }
        }
    }
    panic!("Starting position was not found");
}