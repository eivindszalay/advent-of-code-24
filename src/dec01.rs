use std::fs::read_to_string;
use std::collections::HashMap;



pub fn dec01a() {
    let mut left_ids: Vec<u32> = Vec::new(); 
    let mut right_ids: Vec<u32> = Vec::new(); 
    
    for line in read_to_string("src/in/dec01.in").unwrap().lines() {
        let ids: Vec<&str> = line.split_whitespace().collect();
        left_ids.push(ids[0].parse::<u32>().unwrap());
        right_ids.push(ids[1].parse::<u32>().unwrap());
    }

    left_ids.sort();
    right_ids.sort();

    let mut sum = 0;

    for (index, id) in left_ids.iter().enumerate() {
        sum += id.abs_diff(right_ids[index]);
    }

    println!("{}", sum);
}


pub fn dec01b() {
    let mut left_ids: Vec<u32> = Vec::new(); 
    let mut right_ids: Vec<u32> = Vec::new(); 
    
    for line in read_to_string("src/in/dec01.in").unwrap().lines() {
        let ids: Vec<&str> = line.split_whitespace().collect();
        left_ids.push(ids[0].parse::<u32>().unwrap());
        right_ids.push(ids[1].parse::<u32>().unwrap());
    }
    
    let mut occurences: HashMap<u32, u32> = HashMap::new();

    for id in right_ids.iter() {
        let value = occurences.get(id);

        if let Some(v) = value {
            occurences.insert(*id, v + 1);
        } else {
            occurences.insert(*id, 1);
        }
    }

    let mut sum = 0;

    for id in left_ids.iter() {
        let value = occurences.get(id);
        if let Some(v) = value {
            sum += id*v;
        }
    }

    println!("{}", sum);
    
}