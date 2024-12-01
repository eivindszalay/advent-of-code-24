use std::fs;


pub fn dec01a() {
    println!("Solving the first problem from December 1:");
    let data = fs::read_to_string("src/dec01a.in").expect("Unable to read file");
    println!("{}", data);   
    
}

