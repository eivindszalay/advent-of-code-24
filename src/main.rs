#[allow(dead_code, unused_variables)]
use std::{time::Instant, env::args};
mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec05;
mod dec06;
mod dec07;

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
        dec07::part2
        ].to_vec();

    let args: Vec<String> = args().collect();
    let day_arg_opt = args.get(1);
    let mut day_opt: Option<usize> = None;
    if let Some(d) = day_arg_opt {
        let parsed_arg_result = d.parse::<usize>();
        if let Ok(day) = parsed_arg_result {
            if 0<day && day<=functions.len()/2 {
                day_opt = Some(day)
            }
        }
    }

    execute_functions(functions, day_opt);
}

fn execute_functions(fns: Vec<fn() -> usize>, day: Option<usize>) {
    let before = Instant::now();
    if let Some(d) = day {
        execute_function(fns[(d-1)*2], d, 1);
        execute_function(fns[(d-1)*2+1], d, 2);
    } else {
        for (index, function) in fns.iter().enumerate() {
            execute_function(*function, index/2+1, index%2+1);
        }
    }
    println!("total execution time was {:?}", before.elapsed())
}

fn execute_function(fun: fn() -> usize, day: usize, part: usize) {
    println!("december {}, part {}", day, part);
    let now = Instant::now();
    let solution = fun();
    println!("executed in {:?}, solution is {}\n", now.elapsed(), solution);
}