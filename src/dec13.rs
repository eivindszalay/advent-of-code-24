use std::fs::read_to_string;

type Equation = ((f64, f64, f64), (f64, f64, f64));


/// correct answer is 31897
pub fn part1() -> usize {
    let mut cost = 0;
    for eq in get_equations(false).iter() {
        if let Some((a, b)) = get_solution(*eq) {
            cost += a*3+b;
        }
    }
    cost.try_into().unwrap()
}

/// correct answer is 87596249540359
pub fn part2() -> usize {
    let mut cost = 0;
    for eq in get_equations(true).iter() {
        if let Some((a, b)) = get_solution(*eq) {
            cost += a*3+b;
        }
    }
    cost.try_into().unwrap()
}

fn get_solution(((x_a, x_b, x),(y_a, y_b, y)): Equation) -> Option<(u64, u64)> {
    if y_a==0.0 { return None; }

    let b_denominator: f64 = x_b - y_b*x_a/y_a;
    if round_to_two_decimals(b_denominator) == 0.00 {return None; }

    let b_numerator = x-(y/y_a)*x_a;
    let b = b_numerator/b_denominator;
    let a = (y-b*y_b)/y_a;

    if round_to_two_decimals(b).fract()==0.0 && round_to_two_decimals(a).fract()==0.0 {
       return Some((a.round() as u64, b.round() as u64));
    }
    
    None
} 

fn round_to_two_decimals(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}


fn get_equations(add: bool) -> Vec<Equation> {
    let binding = read_to_string("src/in/dec13.in")
        .unwrap()
        .replace("\r\n", "\n");

    let eq_strings = binding
        .split("\n\n");

    let mut equations = vec!();
    for e in eq_strings {
        let lines: Vec<String> = e
            .split("\n")
            .map(|l| l.to_string())
            .collect();
        let eq_1: Vec<String> = lines[0].split(", ").map(|s| s.to_string()).collect();
        let eq_2: Vec<String> = lines[1].split(", ").map(|s| s.to_string()).collect();
        let sol: Vec<String> = lines[2].split(", ").map(|s| s.to_string()).collect();
        let eq_1_v: Vec<f64> = eq_1.iter().map(get_lh_part).collect();
        let eq_2_v: Vec<f64> = eq_2.iter().map(get_lh_part).collect();
        let sol_v: Vec<f64> = sol.iter().map(|a| get_rh(a, add)).collect();
        equations.push(((eq_1_v[0],eq_2_v[0],sol_v[0] ),(eq_1_v[1],eq_2_v[1],sol_v[1])));


    }
    equations
}


fn get_lh_part(s: &String) -> f64 {
    let foo: String  = s.chars().collect::<Vec<char>>()[s.len()-2..].iter().collect();
    foo.parse().unwrap()
}
fn get_rh(s: &String, add: bool) -> f64 {
    let mut rh = s.split("=").collect::<Vec<&str>>()[1].parse().unwrap();
    if add {
        rh += 10000000000000.0;
    }
    rh
}