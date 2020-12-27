use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day06.txt";

pub fn challenge_1() -> usize {
    let file = File::open(DATA_FILE_PATH).unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut result = 0;
    let mut questions: HashSet<char> = HashSet::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            result += questions.len();
            questions.clear();
        } else {
            for question in line.chars() {
                questions.insert(question);
            }
        }
    }
    result += questions.len();

    return result;
}

pub fn challenge_2() -> usize {
    let file = File::open(DATA_FILE_PATH).unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut result = 0;
    let mut respondents = 0;
    let mut frequencies: HashMap<char, i32> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            for question_frequency in frequencies.values() {
                if *question_frequency == respondents {
                    result += 1;
                }
            }
            respondents = 0;
            frequencies.clear();
        } else {
            let individual: HashSet<char> = line.chars().collect();
            for question in individual {
                let mut value = 1;
                if let Some(previous) = frequencies.get(&question) {
                    value = *previous + 1;
                }
                frequencies.insert(question, value);
            }
            respondents += 1;
        }
    }
    for question_frequency in frequencies.values() {
        if *question_frequency == respondents {
            result += 1;
        }
    }

    return result;
}
