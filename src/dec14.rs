use std::fs::read_to_string;

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
pub fn part2() -> usize {
    0
}

fn parse_line(s: &str) -> ((i32, i32),(i32, i32)) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let pos: Vec<i32> = parts[0].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    let vel: Vec<i32> = parts[1].split("=").nth(1).unwrap().split(",").map(|i| i.parse().unwrap()).collect();
    ((pos[0], pos[1]), (vel[0], vel[1]))

}