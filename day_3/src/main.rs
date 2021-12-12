use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() {
    let mut line_count: i64 = 0;
    let mut col_counts: [i64; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    if let Ok(lines) = read_lines("input/1.txt") {
        for line in lines {
            if let Ok(s) = line {
                line_count += 1;
                let chars = s.split("");
                // println!("{}", s);
                for (i, c) in chars.enumerate() {
                    if i == 0 || i == 13 {
                        continue;
                    }
                    // println!("{}", c);
                    col_counts[i-1] += c.parse::<i64>().unwrap();
                }
            }
        }
    }

    let mut binary_gamma: String = "".to_owned();
    let mut binary_epsilon: String = "".to_owned();
    
    for col_sum in col_counts.iter() {
        if col_sum>&(line_count/2) {
            binary_gamma.push_str("1");
            binary_epsilon.push_str("0");
        } else {
            binary_gamma.push_str("0");
            binary_epsilon.push_str("1");
        }
    }
    let gamma_int = isize::from_str_radix(&binary_gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&binary_epsilon, 2).unwrap();
    println!("Part1: {}", gamma_int*epsilon_int);
}

fn part2() {
    let mut col_counts: [i64; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // let mut col_counts: [i64; 5] = [0, 0, 0, 0, 0];
    let mut oxygen_vec = Vec::new();
    let mut c02_vec = Vec::new();

    if let Ok(lines) = read_lines("input/1.txt") {
        for line in lines {
            if let Ok(s) = line {
                oxygen_vec.push(s.clone());
                c02_vec.push(s);
            }
        }
    }

    let base10: u32 = 10;
    let mut poi = 0;
    while oxygen_vec.len() > 1 {
        let mut poi_sum = 0;
        let mut ones_vec = Vec::new();
        let mut zeros_vec = Vec::new();

        for s in oxygen_vec.iter(){
            let s_vec: Vec<char> = s.chars().collect();
            let s_int = s_vec[poi].to_digit(base10).unwrap();
            if  s_int == 1 {
                ones_vec.push(s.clone());
                poi_sum = poi_sum + 1;
            } else {
                zeros_vec.push(s.clone());
            }
        }

        if poi_sum as f32 >= (oxygen_vec.len() as f32/2.0) {
            oxygen_vec = ones_vec;
        } else {
            oxygen_vec = zeros_vec;
        }
        poi = poi + 1;
    }
    
    let oxygen_val = isize::from_str_radix(&oxygen_vec[0], 2).unwrap();

    let mut poi = 0;
    while c02_vec.len() > 1 {
        let mut poi_sum = 0;
        let mut ones_vec = Vec::new();
        let mut zeros_vec = Vec::new();

        for s in c02_vec.iter(){
            let s_vec: Vec<char> = s.chars().collect();
            let s_int = s_vec[poi].to_digit(base10).unwrap();
            if  s_int == 1 {
                ones_vec.push(s.clone());
                poi_sum = poi_sum + 1;
            } else {
                zeros_vec.push(s.clone());
            }
        }

        if poi_sum as f32 >= (c02_vec.len() as f32/2.0) {
            c02_vec = zeros_vec;
        } else {
            c02_vec = ones_vec;
        }
        poi = poi + 1;
    }
    
    let c02_val = isize::from_str_radix(&c02_vec[0], 2).unwrap();
    println!("Part 2: {}", oxygen_val*c02_val);
}

fn main() {
    part1();
    part2();
}
