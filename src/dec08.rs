use std::{collections::{HashMap, HashSet}, fs::read_to_string};

/// correct answer is 273
pub fn part1() -> usize {
    let input = read_to_string("src/in/dec08.in").unwrap();
    let map: Vec<&str>  = input.lines().collect();
    let height = map.len();
    let length = map[0].len();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            let c: char = map[y].chars().nth(x).unwrap();
            if c != '.' {
                let seen = antennas.get_mut(&c);
                if let Some(s) = seen {
                    s.push((x.try_into().unwrap(), y.try_into().unwrap()));
                } else {
                    antennas.insert(c, vec!((x.try_into().unwrap(), y.try_into().unwrap())));
                }
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for points in antennas.values() {
        for (i, p1) in points.iter().enumerate() {
            for p2 in points[i+1..].iter() {
                antinodes.extend(get_antinodes_pair(*p1, *p2, length.try_into().unwrap(), height.try_into().unwrap()));
            }
        }
    }
    antinodes.len()
}

fn get_antinodes_pair((x1, y1): (i32, i32), (x2, y2): (i32, i32), max_x: i32, max_y: i32 ) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    let diff_x = x2 - x1;
    let diff_y = y2 - y1;
    let mut new_x = x1+diff_x;
    let mut new_y = y1+diff_y;
    if valid(new_x, new_y, max_x, max_y, x2, y2) {
        antinodes.insert((new_x, new_y));
    }
    new_x = x1-diff_x;
    new_y = y1-diff_y;
    if valid(new_x, new_y, max_x, max_y, x2, y2)  {
        antinodes.insert((new_x, new_y));
    }
    new_x = x2+diff_x;
    new_y = y2+diff_y;
    if valid(new_x, new_y, max_x, max_y, x1, y1)  {
        antinodes.insert((new_x, new_y));
    }
    new_x = x2-diff_x;
    new_y = y2-diff_y;
    if valid(new_x, new_y, max_x, max_y, x1, y1)  {
        antinodes.insert((new_x, new_y));
    }
    antinodes
}

fn valid(new_x:i32, new_y:i32, max_x:i32, max_y:i32, x:i32, y:i32) -> bool {
    new_x>=0 && new_x<max_x && new_y>=0 && new_y<max_y && new_x!=x && new_y!=y
}

/// correct answer is 1017
pub fn part2() -> usize {
    let input = read_to_string("src/in/dec08.in").unwrap();
    let map: Vec<&str>  = input.lines().collect();
    let height: usize = map.len();
    let length: usize = map[0].len();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..height {
        for x in 0..length {
            let c: char = map[y].chars().nth(x).unwrap();
            if c != '.' {
                let seen = antennas.get_mut(&c);
                if let Some(s) = seen {
                    s.push((x.try_into().unwrap(), y.try_into().unwrap()));
                } else {
                    antennas.insert(c, vec!( (x.try_into().unwrap(), y.try_into().unwrap())));
                }
            }
        }
    }
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for points in antennas.values() {
        for (i, p1) in points.iter().enumerate() {
            for p2 in points[i+1..].iter() {
                antinodes.extend(get_antinodes_array(*p1, *p2, length.try_into().unwrap(), height.try_into().unwrap()));
            }
        }
    }

    antinodes.len()

}

fn get_antinodes_array((x1, y1): (i32, i32), (x2, y2): (i32, i32), max_x: i32, max_y: i32 ) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();
    let diff_x = x2 - x1;
    let diff_y = y2 - y1;

    let mut x = x1;
    let mut y = y1;
    loop {
        antinodes.insert((x, y));
        x = x + diff_x;
        y = y + diff_y;
        if x<0 || x>=max_x || y< 0 || y>=max_y  {
            break
        }
    }
    x = x1;
    y = y1;
    loop {
        antinodes.insert((x, y));
        x = x - diff_x;
        y = y - diff_y;
        if x<0 || x>=max_x || y< 0 || y>=max_y  {
            break
        }
    }
    antinodes
}