use std::{collections::{HashMap, HashSet}, fs::read_to_string};

/// correct answer 380
pub fn part1() -> usize {
    let bytes: Vec<(u32, u32)> = read_to_string("src/in/dec18.in")
        .unwrap()
        .lines()
        .map(parse_byte)
        .collect();
    let first_kb = bytes[..1024].to_vec();
    let mut current_distance = 0;
    let mut distances: HashMap<(u32, u32), u32> = HashMap::new();
    distances.insert((0,0), current_distance);
    let mut to_visit = vec!(vec!((0,0)));
    while let Some(points) = to_visit.pop() {
        if points.len()==0 {
            break;
        }
        current_distance += 1;
        let mut next: Vec<(u32, u32)> = vec!();
        for (x, y) in points {
            if x>0 && !first_kb.contains(&(x-1, y)) && !distances.contains_key(&(x-1, y)) {
                next.push((x-1, y));
                distances.insert((x-1, y), current_distance);
            }
            if x<70 && !first_kb.contains(&(x+1, y)) && !distances.contains_key(&(x+1, y)) {
                next.push((x+1, y));
                distances.insert((x+1, y), current_distance);
            }
            if y>0 && !first_kb.contains(&(x, y-1)) && !distances.contains_key(&(x, y-1)) {
                next.push((x, y-1));
                distances.insert((x, y-1), current_distance);
            }
            if y<70 && !first_kb.contains(&(x, y+1)) && !distances.contains_key(&(x, y+1)) {
                next.push((x, y+1));
                distances.insert((x, y+1), current_distance);
            }

        }
        to_visit.push(next);
    }

    (*distances.get(&(70,70)).unwrap()).try_into().unwrap()
    
}

/// correct answer is 26,50
pub fn part2() -> usize {
    let bytes: Vec<(u32, u32)> = read_to_string("src/in/dec18.in")
        .unwrap()
        .lines()
        .map(parse_byte)
        .collect();
    let mut fallen_bytes = bytes[..1024].to_vec();
    let mut current_distance = 0;
    let mut distances: HashMap<(u32, u32), u32> = HashMap::new();
    let mut prevs: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    distances.insert((0,0), current_distance);
    let mut to_visit: Vec<Vec<(u32, u32)>> = vec!(vec!((0,0)));
    while let Some(points) = to_visit.pop() {
        if points.len()==0 {
            break;
        }
        current_distance += 1;
        let mut next: Vec<(u32, u32)> = vec!();
        for (x, y) in points {
            if x>0 && !fallen_bytes.contains(&(x-1, y)) && !distances.contains_key(&(x-1, y)) {
                next.push((x-1, y));
                distances.insert((x-1, y), current_distance);
                prevs.insert((x-1, y), (x, y));
            }
            if x<70 && !fallen_bytes.contains(&(x+1, y)) && !distances.contains_key(&(x+1, y)) {
                next.push((x+1, y));
                distances.insert((x+1, y), current_distance);
                prevs.insert((x+1, y), (x, y));
            }
            if y>0 && !fallen_bytes.contains(&(x, y-1)) && !distances.contains_key(&(x, y-1)) {
                next.push((x, y-1));
                distances.insert((x, y-1), current_distance);
                prevs.insert((x, y-1), (x, y));
            }
            if y<70 && !fallen_bytes.contains(&(x, y+1)) && !distances.contains_key(&(x, y+1)) {
                next.push((x, y+1));
                distances.insert((x, y+1), current_distance);
                prevs.insert((x, y+1), (x, y));
            }

        }
        to_visit.push(next);
    }



    let mut path: Vec<(u32, u32)> = vec!((70, 70));
    while path.last().unwrap() != &(0,0) {
        let prev = *prevs.get(path.last().unwrap()).unwrap();
        path.push(prev);
    }

    let mut first = (0,0);
    let last_bytes = bytes[1024..].to_vec();
    for point in last_bytes.iter() {
        fallen_bytes.push(*point);

        let mut been: HashSet<(u32, u32)> = HashSet::new();
        been.insert((0,0));
        let mut to_visit: Vec<Vec<(u32, u32)>> = vec!(vec!((0,0)));
        while let Some(points) = to_visit.pop() {
            if points.len()==0 {
                break;
            }
            let mut next: Vec<(u32, u32)> = vec!();
            for (x, y) in points {
                if x>0 && !fallen_bytes.contains(&(x-1, y)) && !been.contains(&(x-1, y)) {
                    next.push((x-1, y));
                    been.insert((x-1, y));
                }
                if x<70 && !fallen_bytes.contains(&(x+1, y)) && !been.contains(&(x+1, y)) {
                    next.push((x+1, y));
                    been.insert((x+1, y));
                }
                if y>0 && !fallen_bytes.contains(&(x, y-1)) && !been.contains(&(x, y-1)) {
                    next.push((x, y-1));
                    been.insert((x, y-1));
                }
                if y<70 && !fallen_bytes.contains(&(x, y+1)) && !been.contains(&(x, y+1)) {
                    next.push((x, y+1));
                    been.insert((x, y+1));
                }

            }
            to_visit.push(next);
        }
        
        if !been.contains(&(70,70)) {
            first = *point;
            break;
        }
    }
    dbg!(first);
    0
}


fn parse_byte(byte: &str) -> (u32, u32) {
    let coords:Vec<&str> = byte.split(",").collect();
    (coords[0].parse().unwrap(), coords[1].parse().unwrap())
}