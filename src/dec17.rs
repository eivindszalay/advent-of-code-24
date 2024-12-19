use std::fs::read_to_string;

/// correct answer is '1,6,3,6,5,6,5,1,7'
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

/// correct answer is 247839653009594
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

    let mut reg_a_values: Vec<u64> = vec!(0);
    
    for code in program.iter().rev() {

        let mut found_reg_a_values: Vec<u64> = vec!();
        for reg_a in reg_a_values.iter() {

          for index in 0..8 {
              let check = 8*reg_a + index;
              registers[0] = check;

              if get_output(&registers, &program)[0] == *code {
                found_reg_a_values.push(check);
              }
          }
        }
        reg_a_values = found_reg_a_values;

    };

    return reg_a_values[0].try_into().unwrap();
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