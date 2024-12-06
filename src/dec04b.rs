use std::{fmt, fs::read_to_string};

enum Direction {
    NE,
    SE,
    SW,
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

pub fn part2() -> u32 {

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
            if char=='A' {
                let slash = (check_for_char('M', (i, j), Direction::NW, &input) 
                    && check_for_char('S', (i, j), Direction::SE, &input))
                    || (check_for_char('M', (i, j), Direction::SE, &input) 
                    && check_for_char('S', (i, j), Direction::NW, &input));

                let backslash = (check_for_char('M', (i, j), Direction::NE, &input) 
                && check_for_char('S', (i, j), Direction::SW, &input))
                || (check_for_char('M', (i, j), Direction::SW, &input) 
                && check_for_char('S', (i, j), Direction::NE, &input));

                if slash && backslash {sum += 1;}

            }
        }
    }
    return sum;

    
}

fn check_for_char(c: char, starting_point: Point, dir: Direction, input: &Vec<String>) -> bool {
    let width = input.len();
    let height = input[0].len();
    let ( current_x,  current_y) = starting_point;
    if let Ok((x, y)) = next_point(&dir, (current_x, current_y), width, height) {
        let line = input.get(y).unwrap();
        let char = line.chars().nth(x).unwrap();
        if char==c {
                return true; 
        } else { 
            return false 
        }
    } else {
        return false;
    }
}


fn next_point(dir: &Direction, (x, y): Point, w:usize, h:usize) -> Result<Point, OutOfBoundsError> {
    let new_point = match dir {
        Direction::NE => (Some(x+1), y.checked_sub(1)),
        Direction::SE => (Some(x+1), Some(y+1)),
        Direction::SW => (x.checked_sub(1), Some(y+1)),
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
