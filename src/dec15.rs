use std::{collections::HashSet, fs::read_to_string};

/// correct answer is 1442192
pub fn part1() -> usize {
    let input: String = read_to_string("src/in/dec15.in").unwrap().replace("\r\n", "\n");
    let mut map: Vec<Vec<char>> =  input
        .split("\n\n")
        .collect::<Vec<&str>>()[0]
        .lines()
        .map(|l| l
            .chars()
            .collect())
        .collect();
    let (mut x, mut y) = (0,0);
    for i in  0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j]=='@' {
                (x, y) = (j, i);
                break;
            }
        }
    }
    let moves: Vec<&str> =  input.split("\n\n").collect::<Vec<&str>>()[1].lines().collect();
    for line in moves {
        for dir in line.chars() {
            match dir {
                '^' => {
                    let mut i = 1;
                    while map[y-i][x] == 'O' {
                        i += 1;
                    } 
                    if map[y-i][x] == '.' {
                        for j in 2..i+1 {
                            map[y-j][x] = 'O';
                        }
                        map[y][x] = '.';
                        map[y-1][x] = '@';
                        y = y-1;
                    }
                }
                '>' => {
                    let mut i = 1;
                    while map[y][x+i] == 'O' {
                        i += 1;
                    } 
                    if map[y][x+i] == '.' {
                        for j in 2..i+1 {
                            map[y][x+j] = 'O';
                        }
                        map[y][x] = '.';
                        map[y][x+1] = '@';
                        x = x+1;
                    }
                }
                'v' => {
                    let mut i = 1;
                    while map[y+i][x] == 'O' {
                        i += 1;
                    } 
                    if map[y+i][x] == '.' {
                        for j in 2..i+1 {
                            map[y+j][x] = 'O';
                        }
                        map[y][x] = '.';
                        map[y+1][x] = '@';
                        y = y+1;
                    }
                }
                '<' => {
                    let mut i = 1;
                    while map[y][x-i] == 'O' {
                        i += 1;
                    } 
                    if map[y][x-i] == '.' {
                        for j in 2..i+1 {
                            map[y][x-j] = 'O';
                        }
                        map[y][x] = '.';
                        map[y][x-1] = '@';
                        x = x-1;
                    }
                }
                _ => { panic!() }
            }
        }
    }
    let mut sum = 0;
    for y in  0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x]=='O' {
                sum += 100*y + x;
            }
        }
    }

    sum
}

/// correct answer is 1448458
pub fn part2() -> usize {
    let input: String = read_to_string("src/in/dec15.in").unwrap().replace("\r\n", "\n");
    let map: Vec<Vec<char>> =  input
        .split("\n\n")
        .collect::<Vec<&str>>()[0]
        .lines()
        .map(|l| l
            .chars()
            .collect())
        .collect();
    let mut expanded_map: Vec<Vec<char>> = vec!();
    for i in  0..map.len() {
        expanded_map.push(vec!());
        for j in 0..map[0].len() {
            if map[i][j]=='@' {
                expanded_map[i].push('@');
                expanded_map[i].push('.');
            }
            if map[i][j]=='#' {
                expanded_map[i].push('#');
                expanded_map[i].push('#');
            }
            if map[i][j]=='.' {
                expanded_map[i].push('.');
                expanded_map[i].push('.');
            }
            if map[i][j]=='O' {
                expanded_map[i].push('[');
                expanded_map[i].push(']');
            }
        }
    }

    let (mut x, mut y) = (0,0);
    for i in  0..expanded_map.len() {
        for j in 0..expanded_map[0].len() {
            if expanded_map[i][j]=='@' {
                (x, y) = (j, i);
            }
        }
    }
    let moves: Vec<&str> =  input.split("\n\n").collect::<Vec<&str>>()[1].lines().collect();
    for line in moves {
        for dir in line.chars() {
            match dir {
                '^' => {
                    if expanded_map[y-1][x]=='#' {continue;}
                    let mut i = 0;
                    let mut above_boxes: Vec<HashSet<usize>> = vec!();
                    let mut set = HashSet::new();
                    set.insert(x);
                    above_boxes.push(set);
                    while above_boxes.last().unwrap().iter().all(|x| expanded_map[y-i][*x]!='#') && above_boxes.last().unwrap().len()>0 {
                        i += 1;
                        let mut new_above_boxes: HashSet<usize> = HashSet::new();
                        for x in above_boxes.last().unwrap().clone() {
                            if expanded_map[y-i][x]=='[' {
                                new_above_boxes.insert(x);
                                new_above_boxes.insert(x+1);
                            }
                            if expanded_map[y-i][x]==']' {
                                new_above_boxes.insert(x);
                                new_above_boxes.insert(x-1);
                            }
                            if expanded_map[y-i][x]=='#' {
                                new_above_boxes.insert(x);
                            }
                        }
                        above_boxes.push(new_above_boxes);
                    }
                    above_boxes.pop().unwrap(); 
                    if above_boxes.last().unwrap().iter().all(|x| expanded_map[y-i][*x]!='#')  {
                        while above_boxes.len() > 0 {
                            let n = above_boxes.len();
                            let set = above_boxes.pop().unwrap();
                            for x in set.clone() {
                                expanded_map[y-n][x]=expanded_map[y-n+1][x];
                                expanded_map[y-n+1][x] = '.';
                            }
                        }
                        y = y-1;
                    }
                }
                '>' => {
                    if expanded_map[y][x+1]=='#' {continue;}
                    let mut i = 1;
                    while expanded_map[y][x+i] == '[' || expanded_map[y][x+i] == ']' {
                        i += 1;
                    } 
                    if expanded_map[y][x+i] == '.' {
                        for j in 1..i {
                            expanded_map[y][x+i-j+1] = expanded_map[y][x+i-j];
                        }
                        expanded_map[y][x] = '.';
                        expanded_map[y][x+1] = '@';
                        x = x+1;
                    }
                }
                'v' => {
                    if expanded_map[y+1][x]=='#' {continue;}

                    let mut i = 0;
                    let mut above_boxes: Vec<HashSet<usize>> = vec!();
                    let mut set = HashSet::new();
                    set.insert(x);
                    above_boxes.push(set);
                    while above_boxes.last().unwrap().iter().all(|x| expanded_map[y+i][*x]!='#') && above_boxes.last().unwrap().len()>0 {
                        i += 1;
                        let mut new_above_boxes: HashSet<usize> = HashSet::new();
                        for x in above_boxes.last().unwrap().clone() {
                            if expanded_map[y+i][x]=='[' {
                                new_above_boxes.insert(x);
                                new_above_boxes.insert(x+1);
                            }
                            if expanded_map[y+i][x]==']' {
                                new_above_boxes.insert(x);
                                new_above_boxes.insert(x-1);
                            }
                            if expanded_map[y+i][x]=='#' {
                                new_above_boxes.insert(x);
                            }
                        }
                        above_boxes.push(new_above_boxes);
                    }
                    above_boxes.pop().unwrap(); 
                    if above_boxes.last().unwrap().iter().all(|x| expanded_map[y+i][*x]!='#')  {
                        while above_boxes.len() > 0 {
                            let n = above_boxes.len();
                            let set = above_boxes.pop().unwrap();
                            for x in set.clone() {
                                expanded_map[y+n][x]=expanded_map[y+n-1][x];
                                expanded_map[y+n-1][x] = '.';
                            }
                        }
                        y = y+1;
                    }
                }
                '<' => {
                    if expanded_map[y][x-1]=='#' {continue;}
                    let mut i = 1;
                    while expanded_map[y][x-i] == '[' || expanded_map[y][x-i] == ']' {
                        i += 1;
                    } 
                    if expanded_map[y][x-i] == '.' {
                        for j in 1..i {
                            expanded_map[y][x-i+j-1] = expanded_map[y][x-i+j];
                        }
                        expanded_map[y][x] = '.';
                        expanded_map[y][x-1] = '@';
                        x = x-1;
                    }
                }
                _ => { panic!() }
            }

        }
    }
    let mut sum = 0;
    for y in  0..expanded_map.len() {
        for x in 0..expanded_map[0].len() {
            if expanded_map[y][x]=='[' {
                sum += 100*y + x;
            }
        }
    }


    sum
}