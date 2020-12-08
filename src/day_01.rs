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

fn two_sum(numbers: &[i32], target: i32) -> Option<i32> {
    let mut pairs: HashSet<i32> = HashSet::new();
    for number in numbers {
        let needed = target - number;
        if pairs.contains(&needed) {
            return Some(needed * number);
        }
        pairs.insert(*number);
    }
    return None;
}

fn three_sum(numbers: &Vec<i32>, target: i32) -> Option<i32> {
    let mut sorted_numbers = numbers.to_owned();
    sorted_numbers.sort();

    for (position, number) in sorted_numbers.iter().enumerate() {
        let needed = target - number;
        if let Some(two_sum) = two_sum(&sorted_numbers[position + 1..], needed) {
            return Some(number * two_sum);
        }
    }
    return None;
}

pub fn day_01_challenge_1() -> i32 {
    let numbers = load_numbers_from_file(DATA_FILE_PATH);
    return two_sum(&numbers, 2020).unwrap();
}

pub fn day_01_challenge_2() -> i32 {
    let numbers = load_numbers_from_file(DATA_FILE_PATH);
    return three_sum(&numbers, 2020).unwrap();
}
