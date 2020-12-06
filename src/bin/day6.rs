use aoc::*;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let codes = get_codes();
    println!("Day 6 part 1 result is {}", part1(&codes));
    println!("Day 6 part 2 result is {}", part2(&codes));
    Ok(())
}

fn get_codes() -> Vec<String> {
    input("6.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .collect()
}

fn part1(codes: &[String]) -> usize {
    codes
        .iter()
        .map(|s| s.replace(" ", "").chars().collect::<HashSet<char>>().len())
        .sum()
}

fn part2(codes: &[String]) -> usize {
    let mut total = 0;
    for code in codes {
        let mut counts = HashMap::new();
        let users = code.split_whitespace().count();
        for char in code.replace(' ', "").chars() {
            let counter = counts.entry(char).or_insert(0);
            *counter += 1;
        }
        total += counts.values().filter(|v| **v == users).count()
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(&get_codes()) == 6542);
    }

    #[test]
    fn test_part2() {
        assert!(part2(&get_codes()) == 3299);
    }
}
