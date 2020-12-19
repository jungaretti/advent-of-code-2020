use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DATA_FILE_PATH: &str = "./data/day04.txt";

struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

fn passport_from_string(passport_string: &String) -> Passport {
    let mut passport = Passport {
        birth_year: None,
        issue_year: None,
        expiration_year: None,
        height: None,
        hair_color: None,
        eye_color: None,
        passport_id: None,
        country_id: None,
    };

    let passport_fields = passport_string.split_whitespace();
    for field in passport_fields {
        let mut pair = field.split(':');
        let key = pair.next().unwrap();
        let value = pair.next().unwrap();
        match key {
            "byr" => passport.birth_year = Some(String::from(value)),
            "iyr" => passport.issue_year = Some(String::from(value)),
            "eyr" => passport.expiration_year = Some(String::from(value)),
            "hgt" => passport.height = Some(String::from(value)),
            "hcl" => passport.hair_color = Some(String::from(value)),
            "ecl" => passport.eye_color = Some(String::from(value)),
            "pid" => passport.passport_id = Some(String::from(value)),
            "cid" => passport.country_id = Some(String::from(value)),
            _ => (),
        }
    }

    return passport;
}

fn load_passports_from_file(file_path: &str) -> Vec<Passport> {
    let file = File::open(file_path).unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut passports: Vec<Passport> = Vec::new();

    let mut passport_raw = String::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            let passport = passport_from_string(&passport_raw);
            passports.push(passport);
            passport_raw.clear();
        } else {
            passport_raw.push(' ');
            passport_raw.push_str(&line);
        }
    }

    return passports;
}

pub fn day_04_challenge_1() -> usize {
    let passports = load_passports_from_file(DATA_FILE_PATH);
    return passports
        .iter()
        .filter(|passport| {
            passport.birth_year.is_some()
                && passport.issue_year.is_some()
                && passport.expiration_year.is_some()
                && passport.height.is_some()
                && passport.hair_color.is_some()
                && passport.eye_color.is_some()
                && passport.passport_id.is_some()
        })
        .count();
}
