use std::{fmt, fs::read_to_string};

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}


type Point = (usize, usize);
pub struct OutOfBoundsError;

impl fmt::Debug for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutOfBoundsError")
         .finish()
    }
}

pub fn dec04a() {

    let input: Vec<String> = read_to_string("src/in/dec04.in")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    let width = input.len();
    let height = input[0].len();
    let mut sum = 0;
    for j in 0..height {
        let line_chars: Vec<char> = input[j].chars().collect(); // Collect the chars once
        for i in  0..width {
            let char = line_chars[i];
            if char=='X' {
                if check_for_xmas((i, j), Direction::N, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::NE, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::E, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::SE, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::S, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::SW, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::W, &input) {sum += 1;}
                if check_for_xmas((i, j), Direction::NW, &input) {sum += 1;}
            }
        }
    }
    println!("{}", sum);

    
}

fn check_for_xmas(starting_point: Point, dir: Direction, input: &Vec<String>) -> bool {
    let width = input.len();
    let height = input[0].len();
    let mut expected_char = 'M';
    let (mut current_x, mut current_y) = starting_point;
    loop {
        if let Ok((x, y)) = next_point(&dir, (current_x, current_y), width, height) {
            let line = input.get(y).unwrap();
            let char = line.chars().nth(x).unwrap();
            if char==expected_char {
                if char=='S' { return true; }
                expected_char = next_char(char);
                current_x = x;
                current_y = y;
            } else { 
                return false 
            }
        } else {
            return false;
        }
    }
}

fn next_char(c: char) -> char {
    if c== 'X' { return 'M'; }
    if c== 'M' { return 'A'; }
    if c== 'A' { return 'S'; }
    return 'Å';
}


fn next_point(dir: &Direction, (x, y): Point, w:usize, h:usize) -> Result<Point, OutOfBoundsError> {
    let new_point = match dir {
        Direction::N => (Some(x), y.checked_sub(1)), 
        Direction::NE => (Some(x+1), y.checked_sub(1)),
        Direction::E => (Some(x+1), Some(y)),
        Direction::SE => (Some(x+1), Some(y+1)),
        Direction::S => (Some(x), Some(y+1)),
        Direction::SW => (x.checked_sub(1), Some(y+1)),
        Direction::W => (x.checked_sub(1), Some(y)),
        Direction::NW => (x.checked_sub(1), y.checked_sub(1)),
    };

    if let (Some(new_x), Some(new_y)) = new_point {
        if new_x >=w || new_y >=h {
            return Err(OutOfBoundsError);
        }
        return Ok((new_x, new_y));
    } else {
        return Err(OutOfBoundsError);
    }
}
