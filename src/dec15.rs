use std::fs::read_to_string;

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
    dbg!(&map);
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
    for v in map {
        println!("{}", v.iter().collect::<String>());
    }

    sum
}


pub fn part2() -> usize {
    0
}