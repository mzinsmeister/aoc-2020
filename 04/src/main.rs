use std::fs::read_to_string;
use std::collections::BTreeMap;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input = parse_input(&input_string);
    let valid_count = parsed_input.iter().filter(|p| p.is_valid()).count();
    println!("{}", valid_count);
}

struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>
}

impl Passport<'_> {
    fn is_valid(&self) -> bool {
        // Part 1
        /*self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()*/
        // Part 2
        self.byr_is_valid()
            && self.iyr_is_valid()
            && self.eyr_is_valid()
            && self.hgt_is_valid()
            && self.hcl_is_valid()
            && self.ecl_is_valid()
            && self.pid_is_valid()
            && self.cid_is_valid()
    }

    fn byr_is_valid(&self) -> bool {
        if let Some(value) = self.byr {
            return parse_and_check_number(value, 1920, 2002)
        }
        false
    }

    fn iyr_is_valid(&self) -> bool {
        if let Some(value) = self.iyr {
            return parse_and_check_number(value, 2010, 2020)

        }
        false
    }
    fn eyr_is_valid(&self) -> bool {
        if let Some(value) = self.eyr {
            return parse_and_check_number(value, 2020, 2030)
        }
        false
    }
    fn hgt_is_valid(&self) -> bool {
        if let Some(value) = self.hgt {
            let unit = &value[value.len()-2..];
            return match unit {
                "cm" => parse_and_check_number(&value[0..value.len()-2], 150, 193),
                "in" => parse_and_check_number(&value[0..value.len()-2], 59, 76),
                _ => { println!("Unknown unit: {}", unit); false }
            }
        }
        false
    }
    fn hcl_is_valid(&self) -> bool {
        if let Some(value) = self.hcl {
            let first_char = value.chars().next();
            if first_char.is_none() || first_char.unwrap() != '#' { return false; }
            if value.len() != 7 { return false; }
            return value[1..].chars().all(|c| c.is_ascii_hexdigit());
        }
        false
    }
    fn ecl_is_valid(&self) -> bool {
        if let Some(value) = self.ecl {
            let valid_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            return valid_values.contains(&value);
        }
        false
    }
    fn pid_is_valid(&self) -> bool {
        if let Some(value) = self.pid {
            return value.len() == 9 && value.parse::<u32>().is_ok();
        }
        false
    }
    fn cid_is_valid(&self) -> bool {
        true
    }
}

fn parse_and_check_number(string_number: &str, at_least: i32, at_most: i32) -> bool {
    let parsed_number = string_number.parse::<i32>();
    match parsed_number {
        Err(_) => false,
        Ok(number) => number >= at_least && number <= at_most
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(parse_single_passport).collect()
}

fn parse_single_passport(line: &str) -> Passport {
    let map = parse_key_value_pairs(line);
    Passport {
        byr: map.get("byr").map(|e| *e),
        iyr: map.get("iyr").map(|e| *e),
        eyr: map.get("eyr").map(|e| *e),
        hgt: map.get("hgt").map(|e| *e),
        hcl: map.get("hcl").map(|e| *e),
        ecl: map.get("ecl").map(|e| *e),
        pid: map.get("pid").map(|e| *e),
        cid: map.get("cid").map(|e| *e)
    }
}

fn parse_key_value_pairs(line: &str) -> BTreeMap<&str, &str> {
    let mut map = BTreeMap::<&str, &str>::new();
    line.split_whitespace()
        .map(|kv| {
            let mut split = kv.split(":");
            (split.next().unwrap(), split.next().unwrap())
        })
        .for_each(|kv| { map.insert(kv.0, kv.1); () });
    map
}
