use std::fs::read_to_string;

/// correct answer is 231782040
pub fn part1() -> usize {
    let robots: Vec<((i32, i32),(i32, i32))> = read_to_string("src/in/dec14.in")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
    let width = 101;
    let height = 103;
    let mut q_1 = 0;
    let mut q_2 = 0;
    let mut q_3 = 0;
    let mut q_4 = 0;
    for ((x, y),(dx, dy)) in robots {
        let final_x = ((x + 100*dx)%width+width)%width;
        let final_y = ((y + 100*dy)%height+height)%height;
        if final_x<width/2 && final_y<height/2 {
            q_1 += 1;
        }
        if final_x>=width/2+1 && final_y<height/2 {
            q_2 += 1;
        }
        if final_x>=width/2+1 && final_y>=height/2+1 {
            q_3 += 1;
        }
        if final_x<width/2 && final_y>=height/2 +1{
            q_4 += 1;
        }

    }
    q_1*q_2*q_3*q_4
}
/// correct answer is
pub fn part2() -> usize {
    let mut robots: Vec<((i32, i32),(i32, i32))> = read_to_string("src/in/dec14.in")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();
    let mut i = 0;
        println!("{} blinks:", i);
        print_robots(&robots);
        let mut new_bots: Vec<((i32, i32),(i32, i32))> = vec!();
        for (pos, vel) in robots.clone() {
            let new_pos = move_robot(&pos, &vel, i>=11);
            new_bots.push(((new_pos), vel));
        }
        robots = new_bots;
        if i>=11 {
            i += 101;
        } else {
            i += 1;
        }
        println!();
        println!();
        println!();
    6475
}

fn print_robots(robots: &Vec<((i32, i32),(i32, i32))>) {
    let width = 101;
    let height = 103;
    // let width = 11;
    // let height = 7;
    for y in 0..height {
        for x in 0..width {
            if let Some(_) = robots.iter().find(|r| r.0.0==x&&r.0.1==y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}


fn move_robot((x, y):&(i32, i32), (dx, dy): &(i32, i32), big_jump: bool) -> (i32, i32) {
    let width = 101;
    let height = 103;
    // let width = 11;
    // let height = 7;
    let final_x;
    let final_y;
    if !big_jump {
        final_x = ((x + dx)%width+width)%width;
        final_y = ((y + dy)%height+height)%height;
    } else {
        final_x = ((x + dx*101)%width+width)%width;
        final_y = ((y + dy*101)%height+height)%height;
    }
    (final_x, final_y)
}

fn parse_line(s: &str) -> ((i32, i32),(i32, i32)) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let pos: Vec<i32> = parts[0].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    let vel: Vec<i32> = parts[1].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    ((pos[0], pos[1]), (vel[0], vel[1]))

}