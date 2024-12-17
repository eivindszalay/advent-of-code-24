use std::{fs::read_to_string, ops::BitXorAssign};

pub fn part1() -> usize {
    let i = read_to_string("src/in/dec17.in").unwrap();
    let input = i.replace("\r\n", "\n");
    let (r, p) = input.split_once("\n\n").unwrap();

    let mut registers: Vec<u32> = r
        .lines()
        .map(|register| register.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    let program: Vec<u32> = p
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|o| o.parse().unwrap())
        .collect();
    let mut pointer = 0;
    let mut output: Vec<u32> = vec!();
    while pointer < program.len() {
        match program[pointer] {
            0 => adv(&mut registers, &program, &mut pointer),
            1 => bxl(&mut registers, &program, &mut pointer),
            2 => bst(&mut registers, &program, &mut pointer),
            3 => jnz(&mut registers, &program, &mut pointer),
            4 => bxc(&mut registers, &program, &mut pointer),
            5 => out(&mut registers, &program, &mut pointer, &mut output),
            6 => bdv(&mut registers, &program, &mut pointer),
            7 => cdv(&mut registers, &program, &mut pointer),
            _ => break
        }
    }
    0
}

pub fn part2() -> usize {
    0
}

fn adv(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u32;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u32.pow(operand);
    } else {
        denominator = 2_u32.pow(registers[(operand%4) as usize]);
    }
    registers[0] = numerator / denominator;
    *pointer += 2;
}

fn bxl(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    registers[1] = registers[1] ^ program[*pointer+1];
    *pointer += 2;
}
fn bst(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    let operand = program[*pointer+1];
    if operand < 4 {
        registers[1] = operand % 8;
    } else {
        registers[1] = registers[(operand%4) as usize] % 8;
    } 
    *pointer += 2;
}
fn jnz(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    if registers[0] == 0 { 
        *pointer += 2;
        return; 
    }
    *pointer = program[*pointer+1] as usize;
}
fn bxc(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    registers[1] = registers[1] ^ registers[2];
    *pointer += 2;
}
fn out(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize, output: &mut Vec<u32>) {
    let operand = program[*pointer+1];
    if operand < 4 {
        output.push(operand % 8);
    } else {
        output.push(registers[(operand%4) as usize] % 8);
    }
    *pointer += 2;
}
fn bdv(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u32;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u32.pow(operand);
    } else {
        denominator = 2_u32.pow(registers[(operand%4) as usize]);

    }
    registers[1] = numerator / denominator;
    *pointer += 2;
}
fn cdv(registers: &mut Vec<u32>, program: &Vec<u32>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u32;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u32.pow(operand);
    } else {
        denominator = 2_u32.pow(registers[(operand%4) as usize]);

    }
    registers[2] = numerator / denominator;
    *pointer += 2;
}