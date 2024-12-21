use std::{fs::read_to_string, iter::repeat};

pub fn part1() -> usize {
    let input = read_to_string("src/in/dec21.in").unwrap();
    let codes: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for code in codes {
        let sequence = get_human_directional_sequence(code);
        let parsed_code: usize = code.split("A").collect::<Vec<&str>>()[0].parse().unwrap();
        dbg!(sequence.len(), parsed_code);
        println!();
        sum += sequence.len()*parsed_code;
    }
    // let sequence = get_human_directional_sequence("179A");
    // dbg!(sequence.len());

    
    sum
}
pub fn part2() -> usize {
    0
}

fn get_human_directional_sequence(
    code: &str,
) -> String {
    let mut pos = get_directional_keypad_coords('A');

    let my_code = get_outer_robot_directional_sequence(code);

    let mut sequence = "".to_string();


    for button in my_code.chars() {
        let button_coords = get_directional_keypad_coords(button);
        let y_diff = button_coords.1 - pos.1;
        let x_diff = button_coords.0 - pos.0;
        if x_diff<0 && y_diff<0 {
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff<0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff<0 {
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff==0 {
            if y_diff<0 {
                sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            }
        } else if y_diff==0 {
            if x_diff<0 {
                sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            }
        }
        sequence.push('A');
        pos = button_coords;
    }  

    sequence

}

fn get_outer_robot_directional_sequence(
    code: &str,
) -> String {
    let mut pos = get_directional_keypad_coords('A');

    let my_code = get_inner_robot_directional_sequence(code);

    let mut sequence = "".to_string();


    for button in my_code.chars() {
        let button_coords = get_directional_keypad_coords(button);
        let y_diff = button_coords.1 - pos.1;
        let x_diff = button_coords.0 - pos.0;
        if x_diff<0 && y_diff<0 {
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff<0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff<0 {
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff==0 {
            if y_diff<0 {
                sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            }
        } else if y_diff==0 {
            if x_diff<0 {
                sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            }
        }
        sequence.push('A');
        pos = button_coords;
    }    

    sequence


}
fn get_inner_robot_directional_sequence(
    code: &str,
) -> String {
    
    let mut pos = get_numerical_keypad_coords('A');

    let mut sequence = "".to_string();


    for button in code.chars() {
        let button_coords = get_numerical_keypad_coords(button);
        let y_diff = button_coords.1 - pos.1;
        let x_diff = button_coords.0 - pos.0;
        if x_diff<0 && y_diff<0 {
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff<0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff<0 {
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
        } else if x_diff>0 && y_diff>0 {
            sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
        } else if x_diff==0 {
            if y_diff<0 {
                sequence.extend(repeat('^').take(y_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('v').take(y_diff.abs().try_into().unwrap())); 
            }
        } else if y_diff==0 {
            if x_diff<0 {
                sequence.extend(repeat('<').take(x_diff.abs().try_into().unwrap())); 
            } else {
                sequence.extend(repeat('>').take(x_diff.abs().try_into().unwrap())); 
            }
        }

        sequence.push('A');
        pos = button_coords;
    }    
    sequence
}


// fn move_robot(
//     to_button: char,
//     numerical_pad_pos: (i32, i32),
//     directional_pad_1_pos: (i32, i32),
//     directional_pad_2_pos: (i32, i32),
//     directional_pad_3_pos: (i32, i32),
// ) -> String {
//     let (to_x, to_y) = get_numerical_keypad_coords(to_button);
//     "".to_string()
// }

// fn get_diff(
//     (from_x, from_y): (i32, i32),
//     (to_x, to_y): (i32, i32),
// ) -> (i32, i32) {
//     (to_x-from_x,to_y-from_y)
// }

fn get_numerical_keypad_coords(button: char) -> (i32, i32) {
    return match button {
        '0' => { (1, 3) }
        '1' => { (0, 2) }
        '2' => { (1, 2) }
        '3' => { (2, 2) }
        '4' => { (0, 1) }
        '5' => { (1, 1) }
        '6' => { (2, 1) }
        '7' => { (0, 0) }
        '8' => { (1, 0) }
        '9' => { (2, 0) }
        'A' => { (2, 3) }
        '_' => { (0, 3) }
        _ => panic!()
    }
}
fn get_directional_keypad_coords(button: char) -> (i32, i32) {
    return match button {
        '^' => { (1, 0) }
        '>' => { (2, 1) }
        'v' => { (1, 1) }
        '<' => { (0, 1) }
        'A' => { (2, 0) }
        '_' => { (0, 0) }
        _ => panic!()
    }
}