use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day02.txt";

struct Policy {
    character: char,
    minimum: i32,
    maximum: i32,
}

struct Entry {
    policy: Policy,
    password: String,
}

fn load_entires_from_file(file_path: &str) -> Vec<Entry> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| {
            let whole_line = line.unwrap();
            let policy_and_password: Vec<&str> = whole_line.split(':').collect();
            let range_and_character: Vec<&str> = policy_and_password[0].split(" ").collect();
            let minimum_and_maximum: Vec<&str> = range_and_character[0].split("-").collect();

            let minimum = minimum_and_maximum[0].parse::<i32>().unwrap();
            let maximum = minimum_and_maximum[1].parse::<i32>().unwrap();
            let character = range_and_character[1].parse::<char>().unwrap();
            let password = policy_and_password[1].to_owned();

            let policy = Policy {
                character,
                minimum,
                maximum,
            };
            return Entry { policy, password };
        })
        .collect();
}

pub fn day_02_challenge_1() -> i32 {
    let entries = load_entires_from_file(DATA_FILE_PATH);
    return entries
        .iter()
        .filter(|entry| {
            let count = entry
                .password
                .chars()
                .filter(|character| character == &entry.policy.character)
                .count() as i32;
            return count >= entry.policy.minimum && count <= entry.policy.maximum;
        })
        .count() as i32;
}

pub fn day_02_challenge_2() -> i32 {
    let entires = load_entires_from_file(DATA_FILE_PATH);
    return entires
        .iter()
        .filter(|entry| {
            let password = entry.password.trim();
            let min_index = entry.policy.minimum - 1;
            let max_index = entry.policy.maximum - entry.policy.minimum;
            let min_char = password
                .chars()
                .nth((min_index).try_into().unwrap())
                .unwrap();
            let max_char = password
                .chars()
                .nth((max_index).try_into().unwrap())
                .unwrap();

            let min_desired = min_char == entry.policy.character;
            let max_desired = max_char == entry.policy.character;

            return min_desired && !max_desired || !min_desired && max_desired;
        })
        .count() as i32;
}
