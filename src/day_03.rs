use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day03.txt";

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

fn toboggan_travel(forest: &Vec<Vec<ForestEntity>>, slope_x: usize, slope_y: usize) -> usize {
    let mut result = 0;

    let mut pos_x = 0;
    let mut pos_y = 0;
    while pos_y < forest.len() {
        let row = &forest[pos_y];
        let entity = &row[pos_x];
        if *entity == ForestEntity::Tree {
            result += 1
        }
        pos_x += slope_x;
        pos_x %= row.len();
        pos_y += slope_y;
    }

    return result;
}

pub fn day_03_challenge_1() -> usize {
    let forest = load_forest_from_file(DATA_FILE_PATH);
    return toboggan_travel(&forest, 3, 1);
}

pub fn day_03_challenge_2() -> usize {
    let forest = load_forest_from_file(DATA_FILE_PATH);
    let run_1 = toboggan_travel(&forest, 1, 1);
    let run_2 = toboggan_travel(&forest, 3, 1);
    let run_3 = toboggan_travel(&forest, 5, 1);
    let run_4 = toboggan_travel(&forest, 7, 1);
    let run_5 = toboggan_travel(&forest, 1, 2);
    return run_1 * run_2 * run_3 * run_4 * run_5;
}
