use std::fs::read_to_string;

pub fn part1() -> usize {
    let i = read_to_string("src/in/dec17.in").unwrap();
    let input = i.replace("\r\n", "\n");
    let (r, p) = input.split_once("\n\n").unwrap();

    let mut registers: Vec<u64> = r
        .lines()
        .map(|register| register.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    let program: Vec<u64> = p
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|o| o.parse().unwrap())
        .collect();
    let mut pointer = 0;
    let mut output: Vec<u64> = vec!();
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
    let i = read_to_string("src/in/dec17.in").unwrap();
    let input = i.replace("\r\n", "\n");
    let (r, p) = input.split_once("\n\n").unwrap();

    let mut registers: Vec<u64> = r
        .lines()
        .map(|register| register.split_whitespace().last().unwrap().parse().unwrap())
        .collect();
    let program: Vec<u64> = p // kan denne typen endres? disse tallene er aldri høyere enn 7
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|o| o.parse().unwrap())
        .collect();
    registers[0] = 247839571658938;
    dbg!(get_output(&registers, &program));










    registers[0] = 7;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 7, output);
    registers[0] = 56;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 56, output);
    registers[0] = 450;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 450, output);
    registers[0] = 3606;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 3606, output);
    registers[0] = 28852;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 28852, output);
    registers[0] = 230818;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 230818, output);
    registers[0] = 1846548;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 1846548, output);
    registers[0] = 14772389;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 14772389, output);
    registers[0] = 118179114;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 118179114, output);
    registers[0] = 945432931;
    let output = get_output(&registers, &program);
    println!("reg_a: {}, output: {:?}", 945432931, output);
    registers[0] = 7563463490;
    let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 7563463490, output);
    // registers[0] = 60507707920;
    // let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 60507707920, output);
    // registers[0] = 484061663394;
    // let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 484061663394, output);
    // registers[0] = 3872493307170;
    // let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 3872493307170, output);
    // registers[0] = 30979946457367;
    // let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 30979946457367, output);
    // registers[0] = 247839571658938;
    // let output = get_output(&registers, &program);
    // println!("reg_a: {}, output: {:?}", 247839571658938, output);


    let mut copy = program.clone();
    copy.reverse();
    let mut index = 0;
    registers[0] = index;
    let mut important_indices: Vec<u64> = vec!();
    for (i, code) in copy.iter().enumerate() {
        let mut output = get_output(&registers, &program);
        let mut ii = 0;
        while output[0] != *code {
            registers[0] = index + ii;
            output = get_output(&registers, &program);
            if output[0] != *code {
                ii += 1;
            }
        }
        println!("found with reg_a: {}", index+ii);
        important_indices.push(ii);
        index = 0;
        for (iii, e) in important_indices.iter().enumerate() {
            index += 8_u64.pow((i-iii+1) as u32)*e;
        }

        registers[0] = index;
    };

    0
}

fn get_output(registers: &Vec<u64>, program: &Vec<u64>) -> Vec<u64> {
    let mut pointer = 0;
    let mut output: Vec<u64> = vec!();
    let mut r = registers.clone();
    while pointer < program.len() {
        match program[pointer] {
            0 => adv(&mut r, &program, &mut pointer),
            1 => bxl(&mut r, &program, &mut pointer),
            2 => bst(&mut r, &program, &mut pointer),
            3 => jnz(&mut r, &program, &mut pointer),
            4 => bxc(&mut r, &program, &mut pointer),
            5 => out(&mut r, &program, &mut pointer, &mut output),
            6 => bdv(&mut r, &program, &mut pointer),
            7 => cdv(&mut r, &program, &mut pointer),
            _ => break
        }
    }
    output
}

fn adv(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u64;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u64.pow(operand as u32);
    } else {
        denominator = 2_u64.pow((registers[(operand%4) as usize]).try_into().unwrap());
    }
    registers[0] = numerator / denominator;
    *pointer += 2;
}

fn bxl(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    registers[1] = registers[1] ^ program[*pointer+1];
    *pointer += 2;
}
fn bst(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    let operand = program[*pointer+1];
    if operand < 4 {
        registers[1] = operand % 8;
    } else {
        registers[1] = registers[(operand%4) as usize] % 8;
    } 
    *pointer += 2;
}
fn jnz(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    if registers[0] == 0 { 
        *pointer += 2;
        return; 
    }
    *pointer = program[*pointer+1] as usize;
}
fn bxc(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    registers[1] = registers[1] ^ registers[2];
    *pointer += 2;
}
fn out(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize, output: &mut Vec<u64>) {
    let operand = program[*pointer+1];
    if operand < 4 {
        output.push(operand % 8);
    } else {
        output.push(registers[(operand%4) as usize] % 8);
    }
    *pointer += 2;
}
fn bdv(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u64;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u64.pow(operand as u32);
    } else {
        denominator = 2_u64.pow(registers[(operand%4) as usize].try_into().unwrap());

    }
    registers[1] = numerator / denominator;
    *pointer += 2;
}
fn cdv(registers: &mut Vec<u64>, program: &Vec<u64>, pointer: &mut usize ) {
    let numerator = registers[0];
    let denominator: u64;
    let operand = program[*pointer+1];
    if operand < 4 {
        denominator = 2_u64.pow(operand as u32);
    } else {
        denominator = 2_u64.pow(registers[(operand%4) as usize].try_into().unwrap());

    }
    registers[2] = numerator / denominator;
    *pointer += 2;
}