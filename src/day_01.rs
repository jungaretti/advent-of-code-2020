use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day01.txt";

fn load_numbers_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}

fn two_sum(target: i32, numbers: &Vec<i32>, starting_index: usize) -> Option<i32> {
    let mut complements: HashSet<i32> = HashSet::new();
    for number in &numbers[starting_index..] {
        let needed = target - number;
        if complements.contains(&needed) {
            return Some(needed * number);
        }
        complements.insert(*number);
    }
    return None;
}

fn three_sum(target: i32, numbers: &Vec<i32>) -> Option<i32> {
    let mut sorted_numbers = numbers.to_owned();
    sorted_numbers.sort();

    for (pos, number) in sorted_numbers.iter().enumerate() {
        let needed = target - number;
        if let Some(two_sum) = two_sum(needed, &sorted_numbers, pos + 1) {
            return Some(number * two_sum);
        }
    }
    return None;
}

pub fn day_01_challenge_1() -> i32 {
    let numbers = load_numbers_from_file(DATA_FILE_PATH);
    return two_sum(2020, &numbers, 0).unwrap();
}

pub fn day_01_challenge_2() -> i32 {
    let numbers = load_numbers_from_file(DATA_FILE_PATH);
    return three_sum(2020, &numbers).unwrap();
}
