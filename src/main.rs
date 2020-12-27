mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    println!("Day 1");
    println!("- Challenge 1: {}", day_01::day_01_challenge_1());
    println!("- Challenge 2: {}", day_01::day_01_challenge_2());

    println!("Day 2");
    println!("- Challenge 1: {}", day_02::day_02_challenge_1());
    println!("- Challenge 2: {}", day_02::day_02_challenge_2());

    println!("Day 3");
    println!("- Challenge 1: {}", day_03::day_03_challenge_1());
    println!("- Challenge 2: {}", day_03::day_03_challenge_2());

    println!("Day 4");
    println!("- Challenge 1: {}", day_04::day_04_challenge_1());
    println!("- Challenge 2: {}", day_04::day_04_challenge_2());

    println!("Day 5");
    println!("- Challenge 1: {}", day_05::challenge_1());
    println!("- Challenge 2: {}", day_05::challenge_2().unwrap());

    println!("Day 6");
    println!("- Challenge 1: {}", day_06::challenge_1());
    println!("- Challenge 2: {}", day_06::challenge_2());
}
