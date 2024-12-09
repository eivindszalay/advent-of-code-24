use std::{collections::HashMap, fs::read_to_string};

/// correct answer is 6288599492129
pub fn part1() -> usize {
    let disk_map = read_to_string("src/in/dec09.in").unwrap();
    let mut expanded: Vec<String> = vec!();
    for (i, char) in disk_map.chars().enumerate() {
        if i%2==0 {
            let mut file: Vec<String> = vec!();
            for _j in 0..char.to_digit(10).unwrap() {
                let id = i/2;
                file.push(id.to_string());
            }
            expanded.extend(file);
        } else {
            let foo = char.to_digit(10);
            if let Some(e) = foo {
                for _j in 0..e {
                    expanded.push(".".to_string());
                }
            }
        }
    }

    let mut rearranged: Vec<String> = vec!();
    let mut i = 0;
    let mut j = expanded.len()-1;
    while i<=j {
        if expanded.get(i).unwrap()=="." {
            while expanded.get(j).unwrap()=="." {
                j -= 1;
            }
            if j<i {
                break;
            }
            rearranged.push(expanded.get(j).unwrap().clone());
            j -= 1;
            i += 1;
        } else {
            rearranged.push(expanded.get(i).unwrap().clone());
            i += 1;
        }
    }
    let mut sum = 0;
    for (i, id) in rearranged.iter().enumerate() {
        let parsed_id_res = id.parse::<usize>();
        if let Ok(parse_id) = parsed_id_res {
            sum += parse_id*i;
        }
    }
    sum
}
// correct answer is 6321896265143
pub fn part2() -> usize {
    let disk_map = read_to_string("src/in/dec09.in").unwrap();
    let mut expanded: Vec<String> = vec!();
    for (i, char) in disk_map.chars().enumerate() {
        if i%2==0 {
            let mut file: Vec<String> = vec!();
            for _j in 0..char.to_digit(10).unwrap() {
                let id = i/2;
                file.push(id.to_string());
            }
            expanded.extend(file);
        } else {
            let foo = char.to_digit(10);
            if let Some(e) = foo {
                for _j in 0..e {
                    expanded.push(".".to_string());
                }
            }
        }
    }

    let mut rearranged_map: HashMap<String, (usize, usize)> = HashMap::new();
    let mut occupied_map: HashMap<usize, usize> = HashMap::new();
    let mut j = expanded.len()-1;
    while 0<j {
        let id = expanded.get(j).unwrap();
        if id!="." {
            let mut file_length = 0;
            while expanded.get(j).unwrap()==id {
                let opt = j.checked_sub(1);
                if let Some(new_j) = opt {
                    file_length +=1;
                    j = new_j;
                } else {
                    file_length += 1;
                    rearranged_map.insert(id.to_string(), (j, file_length));
                    break;
                }
            }
            let mut k = 0;
            let mut free_length = 0;
            let mut found_room = false;
            while k<j+1 {
                let is_occ_opt = occupied_map.get(&k);
                if let Some(len) = is_occ_opt {
                    k += len;
                    free_length = 0;
                } else {
                    if expanded.get(k).unwrap()=="." || *expanded.get(k).unwrap()==id.to_string() {
                        free_length += 1;
                    } else {
                        free_length = 0;
                    }
                    k += 1;
                } 
                if free_length==file_length {
                    rearranged_map.insert(id.to_string(), (k-file_length, file_length));
                    occupied_map.insert(k-file_length, file_length);
                    found_room = true;
                    break;
                }
            }   
            if !found_room {
                rearranged_map.insert(id.to_string(), (j+1, file_length));
            }
        } else {
            j -= 1;
            if j==0 {
                let id = expanded.get(j).unwrap();
                if id != "." {
                    rearranged_map.insert(id.to_string(), (0, 1));
                }
            }
        }
    }
    let mut sum = 0;
    for (id, (index, length)) in rearranged_map {
        for i in 0..length {
            sum += id.parse::<usize>().unwrap()*(index+i);
        }
    }

    sum
}