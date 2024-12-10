use std::fs::read_to_string;

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

struct File {
    id: usize,
    starting_pos: usize, 
    size: usize,
}

// correct answer is 6321896265143
pub fn part2() -> usize {
    let disk_map: Vec<usize> = read_to_string("src/in/dec09.in")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();
    let mut files: Vec<File> = vec!();
    let mut free: Vec<(usize, usize)> = vec!();
    let mut curr_position = 0;
    for (i, size) in disk_map.iter().enumerate() {
        if i%2==0 {
            files.push(File{id: (i/2), starting_pos: curr_position, size:*size});
        } else {
            free.push(( curr_position,  *size));
        }
        curr_position += size;
    }
    let mut sum = 0;
    while !files.is_empty() {
        let file = files.pop().unwrap();

        let mut free_space = (false, 0, 0, 0);
        for (i, (pos, size)) in free.iter().enumerate() {
            if *pos>=file.starting_pos {
                break;
            }
            if *size>=file.size {
                free_space = (true, i, *pos, *size);
                break;
            }
        }

        if free_space.0 {
            let space_left = free_space.3-file.size;
            free[free_space.1] = (free_space.2+file.size, space_left);
            sum += calculate_file(file);
        } else {
            sum += calculate_file(file);
        }

    }
    return sum;

    fn calculate_file(file: File) -> usize {
        let mut sum = 0;
        for i in 0..file.size {
            sum += file.id*(file.starting_pos+i);
        }
        sum
    }
}