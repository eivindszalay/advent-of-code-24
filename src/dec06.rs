use std::{collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(Eq, Hash, PartialEq, Debug)]
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
    let mut pos = (0, 0);
    
    let mut hori_obsticles: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut vert_obsticles: HashMap<usize, Vec<usize>> = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y].chars().nth(x).unwrap()=='^' {
                pos = (x, y);
            }
            if map[y].chars().nth(x).unwrap()=='#' {
                if let Some(obsticles) = hori_obsticles.get(&y) {
                    let mut clone = obsticles.clone();
                    clone.push(x);
                    hori_obsticles.insert(y, clone);
                } else {
                    hori_obsticles.insert(y, vec!(x));
                }
                if let Some(obsticles) = vert_obsticles.get(&x) {
                    let mut clone = obsticles.clone();
                    clone.push(y);
                    vert_obsticles.insert(x, clone);
                } else {
                    vert_obsticles.insert(x, vec!(y));
                }
            }
        }
    }
    
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
    
    let pos = get_starting_pos(&map);

    let mut sum = 0;
    for (visited_x, visited_y) in path {
        if (visited_x, visited_y) != pos {
            let (hori_obst, vert_obst) = get_obsticles(
                (visited_x, visited_y),
                &hori_obsticles,
                &vert_obsticles
            );
            if has_loop(pos, &hori_obst, &vert_obst) {
                sum += 1;
            }
        }
    }
    sum
}

fn get_obsticles(
    (x, y): (usize, usize),
    hori_obsticles: &HashMap<usize, Vec<usize>>,
    vert_obsticles: &HashMap<usize, Vec<usize>>
) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut hori_obsts = hori_obsticles.clone();
    hori_obsts
        .entry(y)
        .and_modify(|a| {
            if let Some((i, _)) = a.iter().enumerate().find(|(_, e)| *e > &x) {
                a.insert(i, x);
            } else {
                a.push(x);
            }
        })
        .or_insert_with(|| vec![x]);

    let mut vert_obsts = vert_obsticles.clone();
    vert_obsts
        .entry(x)
        .and_modify(|a| {
            if let Some((i, _)) = a.iter().enumerate().find(|(_, e)| *e > &y) {
                a.insert(i, y);
            } else {
                a.push(y);
            }
        })
        .or_insert_with(|| vec![y]);
    (hori_obsts, vert_obsts)
}


fn has_loop(
    (mut x, mut y): (usize, usize),
    hori_obsticles: &HashMap<usize, Vec<usize>>,
    vert_obsticles: &HashMap<usize, Vec<usize>>,
) -> bool {
    let mut dir = &Direction::UP;
    let mut visited_pos: HashSet<(usize, usize, &Direction)> = HashSet::new();
    loop {
        if let Some(_) = visited_pos.get(&(x, y, dir)) {
            return true;
        } else {
            visited_pos.insert((x, y, dir));
        }
        (x, y, dir) = match dir {
            Direction::UP => {
                if let Some(obsticles) = vert_obsticles.get(&x) {
                    if let Some(obsticle) = obsticles.iter().rev().find(|e| e < &&y) {
                        (x, *obsticle+1, &Direction::RIGHT)
                    } else {

                        return false;
                    }
                } else {
                    return false;
                }
            },
            Direction::RIGHT => {
                if let Some(obsticles) = hori_obsticles.get(&y) {
                    if let Some(obsticle) = obsticles.iter().find(|e| e > &&x) {
                        (*obsticle-1, y, &Direction::DOWN)
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            Direction::DOWN => {
                if let Some(obsticles) = vert_obsticles.get(&x) {
                    if let Some(obsticle) = obsticles.iter().find(|e| e > &&y) {
                        (x, *obsticle-1, &Direction::LEFT)
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            Direction::LEFT => {
                if let Some(obsticles) = hori_obsticles.get(&y) {
                    if let Some(obsticle) = obsticles.iter().rev().find(|e| e < &&x) {
                        (*obsticle+1, y, &Direction::UP)
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
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