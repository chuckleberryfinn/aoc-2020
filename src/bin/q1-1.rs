use aoc::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let result = run();
    println!("Day 1 part 1 result is {}", result);
    Ok(())
}

fn run() -> i64 {
    let expenses: HashSet<i64> = input("1.txt")
        .unwrap()
        .lines()
        .map(|e| e.parse().unwrap())
        .collect();

    let complements: HashSet<i64> = expenses.iter().map(|e| complement(*e)).collect();

    expenses.intersection(&complements).product()
}

fn complement(i: i64) -> i64 {
    2020 - i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement() {
        assert!(complement(1) == 2019);
    }

    #[test]
    fn test_run() {
        assert!(run() == 878724);
    }
}
