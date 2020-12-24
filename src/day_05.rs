use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day05.txt";

struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn from_boarding_pass(pass: &str) -> Self {
        let mut row = 0;
        let mut row_max = 127;
        let mut col = 0;
        let mut col_max = 7;

        for direction in pass.chars() {
            let row_mid = (row + row_max) / 2;
            let col_mid = (col + col_max) / 2;

            match direction {
                'F' => row_max = row_mid,
                'B' => row = row_mid + 1,
                'L' => col_max = col_mid,
                'R' => col = col_mid + 1,
                _ => (),
            }
        }

        return Self { row: row, col: col };
    }

    fn id(&self) -> i32 {
        return self.row * 8 + self.col;
    }
}

pub fn challenge_1() -> i32 {
    let file = File::open(DATA_FILE_PATH).unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut max_id = 0;
    while let Some(line) = lines.next() {
        let seat = Seat::from_boarding_pass(&line);
        max_id = cmp::max(seat.id(), max_id);
    }
    return max_id;
}

pub fn challenge_2() -> Option<i32> {
    let file = File::open(DATA_FILE_PATH).unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());

    let max_id = challenge_1();
    let all_ids: HashSet<i32> = lines
        .map(|line| Seat::from_boarding_pass(&line).id())
        .collect();

    for id in 0..=max_id {
        if !all_ids.contains(&id) && all_ids.contains(&(id - 1)) && all_ids.contains(&(id + 1)) {
            return Some(id);
        }
    }

    return None;
}
