use std::path::Path;

use aoc_lib::load;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    pub fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    pub fn valid_simple(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
    pub fn valid_complete(&self) -> bool {
        if !self.valid_simple() {
            return false;
        } else {
            true
        }
    }
    #[rustfmt::skip]
    pub fn match_attribute(&mut self, attribute: &str, value: &str) {
        match attribute {
            "byr" => {self.byr = Some(value.to_string());}
            "iyr" => {self.iyr = Some(value.to_string());}
            "eyr" => {self.eyr = Some(value.to_string());}
            "hgt" => {self.hgt = Some(value.to_string());}
            "hcl" => {self.hcl = Some(value.to_string());}
            "ecl" => {self.ecl = Some(value.to_string());}
            "pid" => {self.pid = Some(value.to_string());}
            "cid" => {self.cid = Some(value.to_string());}
            _ => {}
        }
    }
}

fn parse_ports<P: AsRef<Path> + std::fmt::Debug + Copy>(
    path: P,
    filter: fn(&Passport) -> bool,
) -> usize {
    load(path)
        .split("\n\n")
        .map(|s| {
            let mut passport = Passport::new();
            s.split(|c| c == ' ' || c == '\n')
                .filter(|s| !s.is_empty())
                .for_each(|prop| {
                    let mut split = prop.split(':');
                    let attribute = split.next().unwrap();
                    let value = split.next().unwrap();
                    passport.match_attribute(attribute, value);
                });
            passport
        })
        .filter(filter)
        .count()
}

pub fn part1() -> usize {
    parse_ports("input/day4.txt", |p| p.valid_simple())
}
pub fn part2() -> usize {
    parse_ports("input/day4.txt", |p| p.valid_complete())
}
