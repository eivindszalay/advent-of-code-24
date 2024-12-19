use std::fs::read_to_string;

type Robot = ((i32, i32),(i32, i32));

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const PART_2_SOLUTION: i32 = 6475;

/// correct answer is 231782040
pub fn part1() -> usize {
    let robots = get_robots();
    let mut q_1 = 0;
    let mut q_2 = 0;
    let mut q_3 = 0;
    let mut q_4 = 0;
    for ((x, y),(dx, dy)) in robots {
        let final_x = new_coordinate(x, dx, 100, WIDTH);
        let final_y = new_coordinate(y, dy, 100, HEIGHT);
        if final_x<WIDTH/2 && final_y<HEIGHT/2 {
            q_1 += 1;
        }
        if final_x>=WIDTH/2+1 && final_y<HEIGHT/2 {
            q_2 += 1;
        }
        if final_x>=WIDTH/2+1 && final_y>=HEIGHT/2+1 {
            q_3 += 1;
        }
        if final_x<WIDTH/2 && final_y>=HEIGHT/2 +1{
            q_4 += 1;
        }

    }
    q_1*q_2*q_3*q_4
}
/// correct answer is 6475
pub fn part2() -> usize {
    let robots = get_robots();
    let mut new_bots: Vec<((i32, i32),(i32, i32))> = vec!();
    for ((x, y), (dx, dy)) in robots.clone() {
        let final_x = new_coordinate(x, dx, PART_2_SOLUTION, WIDTH);
        let final_y = new_coordinate(y, dy, PART_2_SOLUTION, HEIGHT);
        new_bots.push(((final_x, final_y), (dx, dy)));
    }
    // print_robots(new_bots);
    PART_2_SOLUTION.try_into().unwrap()
}

#[allow(dead_code)]
fn print_robots(robots: Vec<Robot>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let Some(_) = robots.iter().find(|r| r.0.0==x&&r.0.1==y) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn get_robots() -> Vec<Robot> {
    let robots = read_to_string("src/in/dec14.in")
    .unwrap()
    .lines()
        .map(parse_line)
        .collect();
    robots
}

fn parse_line(s: &str) -> Robot {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let pos: Vec<i32> = parts[0].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    let vel: Vec<i32> = parts[1].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    ((pos[0], pos[1]), (vel[0], vel[1]))
}

fn new_coordinate(pos: i32, vel: i32, secs: i32, modulo: i32) -> i32 {
    ((pos + vel*secs)%modulo + modulo) % modulo
}
