use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day03.txt";

const SLOPE_X: usize = 3;
const SLOPE_Y: usize = 1;

#[derive(PartialEq)]
enum ForestEntity {
    Square,
    Tree,
}

fn char_to_entity(entity: char) -> ForestEntity {
    match entity {
        '.' => ForestEntity::Square,
        _ => ForestEntity::Tree,
    }
}

fn load_forest_from_file(file_path: &str) -> Vec<Vec<ForestEntity>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    return reader
        .lines()
        // Build the forest line by line
        .map(|line| line.unwrap().chars().map(char_to_entity).collect())
        .collect();
}

pub fn day_03_challenge_1() -> i32 {
    let forest = load_forest_from_file(DATA_FILE_PATH);

    let mut result = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;
    while pos_y < forest.len() {
        let row = &forest[pos_y];
        let entity = &row[pos_x];

        if *entity == ForestEntity::Tree {
            result += 1
        }

        pos_x += SLOPE_X;
        pos_x %= row.len();
        pos_y += SLOPE_Y;
    }

    return result;
}
