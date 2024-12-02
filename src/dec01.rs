use std::fs::read_to_string;


pub fn dec01a() {
    let mut left_ids: Vec<u32> = Vec::new(); 
    let mut right_ids: Vec<u32> = Vec::new(); 

    for line in read_to_string("src/dec01a.in").unwrap().lines() {
        let ids: Vec<&str> = line.split_whitespace().collect();
        left_ids.push(ids[0].parse::<u32>().unwrap());
        right_ids.push(ids[1].parse::<u32>().unwrap());
    }

    left_ids.sort();
    right_ids.sort();
    println!("{}", left_ids.first().unwrap());
    println!("{}", left_ids.last().unwrap());

    let mut sum = 0;

    for (index, id) in left_ids.iter().enumerate() {
        sum += id.abs_diff(right_ids[index]);
    }

    println!("{}", sum);
}
