use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut horizontal: i64 = 0;
    let mut vertical: i64 = 0;
    let mut aim: i64 = 0;
    if let Ok(lines) = read_lines("input1.txt") {
        for line in lines {
            if let Ok(s) = line {
                let split_vec: Vec<&str> = s.split(" ").collect();
                
                if split_vec[0] == "forward" {
                    horizontal = horizontal + split_vec[1].parse::<i64>().unwrap();
                    vertical = vertical + aim*split_vec[1].parse::<i64>().unwrap();
                }
                
                if split_vec[0] == "down" {
                    aim = aim + split_vec[1].parse::<i64>().unwrap();
                }

                if split_vec[0] == "up" {
                    aim = aim - split_vec[1].parse::<i64>().unwrap();
                }
            }
        }
    }
    println!("Guess: {}", horizontal*vertical)
}