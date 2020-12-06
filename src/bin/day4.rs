use aoc::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let passports = get_passports();
    println!("Day 4 part 1 result is {}", part1(&passports));
    println!("Day 4 part 2 result is {}", part2(&passports));
    Ok(())
}

fn get_passports() -> Vec<HashMap<String, String>> {
    let x: Vec<Vec<String>> = input("4.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .map(|s| {
            s.split(' ')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();

    let mut passports = Vec::new();
    for row in x.iter() {
        let mut passport: HashMap<String, String> = HashMap::new();
        for entry in row.iter() {
            let kv: (&str, &str) = entry.splitn(2, ':').collect_tuple().unwrap();
            passport.insert(kv.0.to_string(), kv.1.to_string());
        }
        passports.push(passport);
    }
    passports
}

fn get_required_fields() -> HashMap<String, regex::Regex> {
    [
        (
            "byr".to_string(),
            Regex::new(r"^19[2-9]\d|200[0-2]$").unwrap(),
        ),
        ("iyr".to_string(), Regex::new(r"^201\d|2020$").unwrap()),
        ("eyr".to_string(), Regex::new(r"^202\d|2030$").unwrap()),
        (
            "hgt".to_string(),
            Regex::new(r"^(1[5-8]\d|19[0-3])cm$|^(59|6\d|7[0-6])in$").unwrap(),
        ),
        ("hcl".to_string(), Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
        (
            "ecl".to_string(),
            Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap(),
        ),
        ("pid".to_string(), Regex::new(r"^\d{9}$").unwrap()),
    ]
    .iter()
    .cloned()
    .collect()
}

fn valid_part1(passports: &[HashMap<String, String>]) -> Vec<HashMap<String, String>> {
    let requried_keys = &get_required_fields()
        .keys()
        .cloned()
        .collect::<HashSet<String>>();

    passports
        .iter()
        .filter(|p| {
            p.keys()
                .cloned()
                .collect::<HashSet<String>>()
                .intersection(&requried_keys)
                .count()
                == 7
        })
        .cloned()
        .collect()
}

fn part1(passports: &[HashMap<String, String>]) -> usize {
    valid_part1(passports).len()
}

fn part2(passports: &[HashMap<String, String>]) -> usize {
    let mut count = 0;
    let required = &get_required_fields();
    let valid = valid_part1(passports);
    for vp1 in valid {
        let mut is_valid = true;
        for (k, v) in vp1.iter() {
            if let Some(regex) = required.get(k) {
                if !regex.is_match(v) {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(&get_passports()) == 260);
    }

    #[test]
    fn test_part2() {
        assert!(part2(&get_passports()) == 153);
    }
}
