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

impl Passport {
    fn is_full(&self) -> bool {
        return self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some();
    }

    fn is_valid(&self) -> bool {
        let hgt_regex = Regex::new(r"^(((1[5-8]\d)|(19[0-3]))cm|(59|[6-7][0-9])in)$").unwrap();
        let hcl_regex = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();
        let ecl_regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
        let pid_regex = Regex::new(r"^\d{9}$").unwrap();

        let mut byr_valid = false;
        if let Some(year) = &self.birth_year {
            if let Ok(year) = year.parse::<i32>() {
                byr_valid = year >= 1920 && year <= 2002;
            }
        }
        let mut iyr_valid = false;
        if let Some(year) = &self.issue_year {
            if let Ok(year) = year.parse::<i32>() {
                iyr_valid = year >= 2010 && year <= 2020;
            }
        }
        let mut eyr_valid = false;
        if let Some(year) = &self.expiration_year {
            if let Ok(year) = year.parse::<i32>() {
                eyr_valid = year >= 2020 && year <= 2030;
            }
        }
        let mut hgt_valid = false;
        if let Some(height) = &self.height {
            hgt_valid = hgt_regex.is_match(height);
        }
        let mut hcl_valid = false;
        if let Some(hair_color) = &self.hair_color {
            hcl_valid = hcl_regex.is_match(hair_color);
        }
        let mut ecl_valid = false;
        if let Some(eye_color) = &self.eye_color {
            ecl_valid = ecl_regex.is_match(eye_color);
        }
        let mut pid_valid = false;
        if let Some(passport_id) = &self.passport_id {
            pid_valid = pid_regex.is_match(passport_id);
        }

        return byr_valid
            && iyr_valid
            && eyr_valid
            && hgt_valid
            && hcl_valid
            && ecl_valid
            && pid_valid;
    }
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

    for field in passport_string.split_whitespace() {
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
        .filter(|passport| passport.is_full())
        .count();
}

pub fn day_04_challenge_2() -> usize {
    let passports = load_passports_from_file(DATA_FILE_PATH);
    return passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();
}
