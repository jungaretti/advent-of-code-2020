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
