use std::{cmp, collections::{HashMap, HashSet}, fs::read_to_string};

/// correct answer is 273
pub fn part1() -> usize {
    let input = read_to_string("src/in/dec08.in").unwrap();
    let map: Vec<&str>  = input.lines().collect();
    let height = map.len();
    let length = map[0].len();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            let c: char = map[y].chars().nth(x).unwrap();
            if c != '.' {
                let seen = antennas.get_mut(&c);
                if let Some(s) = seen {
                    s.push((x, y));
                } else {
                    antennas.insert(c, vec!( (x, y)));
                }
            }
        }
    }
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (symbol, points) in antennas {
        println!("{:?}", (symbol, &points));
        for (i, (x1, y1)) in points.iter().enumerate() {
            for (x2, y2) in points[i+1..].iter() {
                let greater_x = cmp::max(x1, x2);
                let smaller_x = cmp::min(x1, x2);
                let x_diff = greater_x - smaller_x;
                let greater_y = cmp::max(y1, y2);
                let smaller_y = cmp::min(y1, y2);
                let y_diff = greater_y - smaller_y;

                // point 1
                {
                    if x1 == greater_x {
                        let new_x = add(*x1, x_diff, length);
                        if y1 == greater_y {
                            let new_y = add(*y1, y_diff, height);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        } else {
                            let new_y = y1.checked_sub(y_diff);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        }
                    } else {
                        let new_x = x1.checked_sub(x_diff);
                        if y1 == greater_y {
                            let new_y = add(*y1, y_diff, height);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        } else {
                            let new_y = y1.checked_sub(y_diff);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        }
                    }
                }
                // point 2
                {
                    if x2 == greater_x {
                        let new_x = add(*x2, x_diff, length);
                        if y2 == greater_y {
                            let new_y = add(*y2, y_diff, height);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        } else {
                            let new_y = y2.checked_sub(y_diff);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        }
                    } else {
                        let new_x = x2.checked_sub(x_diff);
                        if y2 == greater_y {
                            let new_y = add(*y2, y_diff, height);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        } else {
                            let new_y = y2.checked_sub(y_diff);
                            if let Some(x) = new_x {
                                if let Some(y) = new_y {
                                    antinodes.insert((x, y));
                                }
                            }
                        }
                    }
                }

            }
        }
    }




    antinodes.len()
}

fn add(c1: usize, c2: usize, max: usize) -> Option<usize> {
    let sum = c1+c2;
    if sum >= max {return None;}
    return Some(sum);
}

pub fn part2() -> usize {
    0
}