#[allow(dead_code, unused_variables)]
use std::{time::Instant, env::args};
mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;
mod dec06;
mod dec07;
mod dec08;
mod dec09;
mod dec10;
mod dec11;
mod dec12;
mod dec13;
mod dec14;
mod dec15;

fn main() {
    
    let functions = [
        dec01::part1,
        dec01::part2, 
        dec02::part1,
        dec02::part2, 
        dec03::part1,
        dec03::part2,
        dec04::part1,
        dec04::part2,
        dec05::part1,
        dec05::part2,
        dec06::part1,
        dec06::part2,
        dec07::part1,
        dec07::part2,
        dec08::part1,
        dec08::part2,
        dec09::part1,
        dec09::part2,
        dec10::part1,
        dec10::part2,
        dec11::part1,
        dec11::part2,
        dec12::part1,
        dec12::part2,
        dec13::part1,
        dec13::part2,
        dec14::part1,
        dec14::part2,
        dec15::part1,
        dec15::part2,
        ].to_vec();

    let args: Vec<String> = args().collect();
    let day_arg_opt = args.get(1);
    let mut day_opt = None;
    if let Some(day_arg) = day_arg_opt {
        let day_arg_res = day_arg.parse();
        if let Ok(day) = day_arg_res {
            if 0<day && day<=functions.len()/2 {
                day_opt = Some(day)
            }
        }
    }

    let before = Instant::now();
    if let Some(d) = day_opt {
        execute_function(functions[(d-1)*2], d, 1);
        execute_function(functions[(d-1)*2+1], d, 2);
    } else {
        for (index, fun) in functions.iter().enumerate() {
            execute_function(*fun, index/2+1, index%2+1);
        }
    }
    println!("total execution time was {:?}", before.elapsed());
    print!("\x07");
}

fn execute_function(fun: fn() -> usize, day: usize, part: usize) {
    println!("december {}, part {}", day, part);
    let now = Instant::now();
    let solution = fun();
    println!("executed in {:?}, solution is {}\n", now.elapsed(), solution);
}