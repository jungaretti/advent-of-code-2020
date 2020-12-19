use regex::Regex;
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

struct VerifiedPassport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<i32>,
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

fn verified_passport_from_string(passport_string: &String) -> VerifiedPassport {
    let mut passport = VerifiedPassport {
        birth_year: None,
        issue_year: None,
        expiration_year: None,
        height: None,
        hair_color: None,
        eye_color: None,
        passport_id: None,
        country_id: None,
    };

    let hgt_regex = Regex::new(r"^(((1[5-8]\d)|(19[0-3]))cm|(59|[6-7][0-9])in)$").unwrap();
    let hcl_regex = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();
    let ecl_regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();

    let passport_fields = passport_string.split_whitespace();
    for field in passport_fields {
        let mut pair = field.split(':');
        let key = pair.next().unwrap();
        let value = pair.next().unwrap();
        match key {
            "byr" => {
                if let Ok(year) = value.parse::<i32>() {
                    if year >= 1920 && year <= 2002 {
                        passport.birth_year = Some(year);
                    }
                }
            }
            "iyr" => {
                if let Ok(year) = value.parse::<i32>() {
                    if year >= 2010 && year <= 2020 {
                        passport.issue_year = Some(year);
                    }
                }
            }
            "eyr" => {
                if let Ok(year) = value.parse::<i32>() {
                    if year >= 2020 && year <= 2030 {
                        passport.expiration_year = Some(year);
                    }
                }
            }
            "hgt" => {
                if hgt_regex.is_match(value) {
                    passport.height = Some(String::from(value));
                }
            }
            "hcl" => {
                if hcl_regex.is_match(value) {
                    passport.hair_color = Some(String::from(value));
                }
            }
            "ecl" => {
                if ecl_regex.is_match(value) {
                    passport.eye_color = Some(String::from(value));
                }
            }
            "pid" => {
                if pid_regex.is_match(value) {
                    passport.passport_id = Some(value.parse::<i32>().unwrap());
                }
            }
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

fn load_verified_passports_from_file(file_path: &str) -> Vec<VerifiedPassport> {
    let file = File::open(file_path).unwrap();
    let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let mut passports: Vec<VerifiedPassport> = Vec::new();

    let mut passport_raw = String::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            let passport = verified_passport_from_string(&passport_raw);
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

pub fn day_04_challenge_2() -> usize {
    let passports = load_verified_passports_from_file(DATA_FILE_PATH);
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
