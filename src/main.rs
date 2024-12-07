#[allow(dead_code, unused_variables)]
use std::time::Instant;
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
        dec06::part2
        ].to_vec();
    
    // execute_functions(functions);
    println!("{}", dec07::part1());
    println!("{}", dec07::part2());
}

fn execute_functions(fns: Vec<fn() -> u32>) {
    let before = Instant::now();
    for (index, function) in fns.iter().enumerate() {
        println!("december {}, part {}", index/2+1, index%2+1);
        let now = Instant::now();
        let solution = function();
        println!("executed in {:?}, solution is {}\n", now.elapsed(), solution);
    }
    println!("total execution time was {:?}", before.elapsed())
}