#[allow(dead_code)]
use std::time::Instant;
mod dec01;
mod dec02;
mod dec03;

fn main() {
    
    let functions = [
        dec01::dec01a,
        dec01::dec01b, 
        dec02::dec02a,
        dec02::dec02b, 
        dec03::dec03a,
        dec03::dec03b
    ].to_vec();
    
    execute_functions(functions);
}

fn execute_functions(fns: Vec<fn()>) {
    let mut now = Instant::now();
    for (index, function) in fns.iter().enumerate() {
        println!("december {}, part {}", index/2+1, index%2+1);
        function();
        println!("executed in {:?}\n", now.elapsed());
        now = Instant::now();
    }
}