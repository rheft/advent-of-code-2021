use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    return numbers;
}

fn main() {
    let file_input = load_from_file("input1.txt");
    let mut increase_count: u32 = 0;
    let mut current_value: i64 = 0;
    for (i,  val) in file_input.iter().enumerate() {
        if val > &current_value && i != 0 {
            increase_count = increase_count + 1;
        }
        
        current_value = *val;
    }

    println!("Total increases: {}", increase_count)
}