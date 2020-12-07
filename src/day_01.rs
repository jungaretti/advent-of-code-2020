use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const TARGET: i32 = 2020;
const DATA_FILE_PATH: &str = "./txt/day01.txt";

fn load_numbers_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        numbers.push(line.unwrap().parse::<i32>().unwrap())
    }
    return numbers;
}

pub fn day_01_challenge_1() -> i32 {
    let numbers = load_numbers_from_file(DATA_FILE_PATH);
    let mut pairs: HashSet<i32> = HashSet::new();

    for number in numbers {
        let needed = TARGET - number;
        if pairs.contains(&needed) {
            return needed * number;
        }
        pairs.insert(number);
    }
    
    return -1;
}

pub fn day_01_challenge_2() -> i32 {
    return -1;
}
