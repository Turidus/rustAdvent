mod rexes {
    pub use regex::Regex;

    pub struct Rexes {
        pub blank: Regex,
        pub byr: Regex,
        pub iyr: Regex,
        pub eyr: Regex,
        pub hgt: Regex,
        pub hcl: Regex,
        pub ecl: Regex,
        pub pid: Regex,
        pub cid: Regex
    }

    pub fn get_simple_rexes() -> Rexes {
        Rexes {
            blank: Regex::new(r"^\s*$").unwrap(),
            byr: Regex::new(r"byr:([^\s]+)").unwrap(),
            iyr: Regex::new(r"iyr:([^\s]+)").unwrap(),
            eyr: Regex::new(r"eyr:([^\s]+)").unwrap(),
            hgt: Regex::new(r"hgt:([^\s]+)").unwrap(),
            hcl: Regex::new(r"hcl:([^\s]+)").unwrap(),
            ecl: Regex::new(r"ecl:([^\s]+)").unwrap(),
            pid: Regex::new(r"pid:([^\s]+)").unwrap(),
            cid: Regex::new(r"cid:([^\s]+)").unwrap(),
        }
    }

    pub fn get_complex_rexes() -> Rexes {
        Rexes {
            blank: Regex::new(r"^\s*$").unwrap(),
            byr: Regex::new(r"byr:(\d{4})\s").unwrap(),
            iyr: Regex::new(r"iyr:(\d{4})\s").unwrap(),
            eyr: Regex::new(r"eyr:(\d{4})\s").unwrap(),
            hgt: Regex::new(r"hgt:(\d+)(cm|in)\s").unwrap(),
            hcl: Regex::new(r"hcl:#[0-9a-f]{6}\s").unwrap(),
            ecl: Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s").unwrap(),
            pid: Regex::new(r"pid:\d{9}\s").unwrap(),
            cid: Regex::new(r"cid:([^\s]+)\s").unwrap(),
        }
    }

    pub fn valid(passport: &str) -> bool {
        lazy_static!{
            static ref RE: Rexes = get_simple_rexes();
        }

        RE.byr.is_match(&passport)
            && RE.iyr.is_match(&passport)
            && RE.eyr.is_match(&passport)
            && RE.hcl.is_match(&passport)
            && RE.hgt.is_match(&passport)
            && RE.ecl.is_match(&passport)
            && RE.pid.is_match(&passport)
    }

    pub fn valid_data(passport: &str) -> bool {
        lazy_static! {
            static ref RE: Rexes = get_complex_rexes();
        }

        if !valid(passport) {return false}
        //byr
        let cap = match RE.byr.captures(passport){
            None => {return false}
            Some(c) => {c}
        };
        let year = cap.get(1).unwrap().as_str();
        //print!("bry: {} ", cap.get(0).unwrap().as_str());
        match year.parse::<u32>() {
            Ok(num) => {
                if num < 1920 || num > 2020 {return false}}
            Err(_) => {return false}
        };
        //iyr
        let cap = match RE.iyr.captures(passport){
            None => {return false}
            Some(c) => {c}
        };
        //print!("iry: {} ", cap.get(0).unwrap().as_str());
        let year = cap.get(1).unwrap().as_str();
        match year.parse::<u32>() {
            Ok(num) => {if num < 2010 || num > 2020 {return false}}
            Err(_) => {return false}
        };
        //eyr
        let cap = match RE.eyr.captures(passport){
            None => {return false}
            Some(c) => {c}
        };
        //print!("ery: {} ", cap.get(0).unwrap().as_str());
        let year = cap.get(1).unwrap().as_str();
        match year.parse::<u32>() {
            Ok(num) => {if num < 2020 || num > 2030 {return false}}
            Err(_) => {return false}
        };
        //hgt
        let cap = match RE.hgt.captures(passport){
            None => {return false}
            Some(c) => {c}
        };
        //print!("hgt: {} ", cap.get(0).unwrap().as_str());
        let unit = cap.get(2).unwrap().as_str();
        let height = cap.get(1).unwrap().as_str();
        match height.parse::<u32>() {
            Err(_) => {return false}
            Ok(c) => {
                if unit == "cm" { if c < 150 || c > 193 {return false}}
                else {if c < 59 || c > 76 {return false}}
            }
        };

        RE.hcl.is_match(passport)
            && RE.ecl.is_match(passport)
            && RE.pid.is_match(passport)
    }
}

pub mod puzzle_one {
    use std::fs;
    use std::path::Path;
    use crate::day_four::rexes::*;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re = get_simple_rexes();
        let mut count = 0;
        let mut passport = String::from("");

        for line in input.lines() {
            if !re.blank.is_match(line) {
                passport.push_str(line);
                passport.push(' ');
            }
            else {
                if valid(&passport) {count += 1;}
                passport = String::from("");
            }
        }
        if !re.blank.is_match(&passport){
            if valid(&passport) {count += 1;}
        }
        println!("Answer is {}", count)
    }
}


pub mod puzzle_two {
    use std::fs;
    use std::path::Path;
    use crate::day_four::rexes::*;

    pub fn run(path: &Path){
        let input = fs::read_to_string(path)
            .expect("Reading the file was not possible.");

        let re = get_simple_rexes();
        let mut count = 0;
        let mut passport = String::from("");

        for line in input.lines() {
            if !re.blank.is_match(line) {
                passport.push_str(line);
                passport.push(' ');
            }
            else {
                if valid_data(&passport) {count += 1;}
                passport = String::from("");
            }
        }
        if !re.blank.is_match(&passport){
            if valid_data(&passport) {count += 1;}
        }
        println!("Answer is {}", count)
    }
}