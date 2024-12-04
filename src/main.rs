#[allow(dead_code)]
use std::time::Instant;
mod dec01;
mod dec02;
mod dec03;
mod dec04;
mod dec04b;

fn main() {
    
    let functions = [
        dec01::dec01a,
        dec01::dec01b, 
        dec02::dec02a,
        dec02::dec02b, 
        dec03::dec03a,
        dec03::dec03b,
        dec04::dec04a,
        dec04b::dec04b
        ].to_vec();
        
    execute_functions(functions);
}

fn execute_functions(fns: Vec<fn()>) {
    for (index, function) in fns.iter().enumerate() {
        println!("december {}, part {}", index/2+1, index%2+1);
        let now = Instant::now();
        function();
        println!("executed in {:?}\n", now.elapsed());
    }
}