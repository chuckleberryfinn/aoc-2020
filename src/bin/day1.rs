use aoc::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    println!("Day 1 part 1 result is {}", part1());
    println!("Day 1 part 2 result is {}", part2());
    Ok(())
}

fn get_hashset() -> HashSet<i64> {
    input("1.txt")
        .unwrap()
        .lines()
        .map(|e| e.parse().unwrap())
        .collect()
}

fn part1() -> i64 {
    let expenses = get_hashset();
    let complements: HashSet<i64> = expenses.iter().map(|e| complement(*e)).collect();
    expenses.intersection(&complements).product()
}

fn part2() -> i64 {
    let expenses_hash = get_hashset();
    let expenses: Vec<i64> = expenses_hash.iter().copied().collect();
    let mut solution = -1;

    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            let (x, y) = (expenses[i], expenses[j]);
            let z = complement(x + y);
            if expenses_hash.contains(&z) {
                solution = z * x * y;
                break;
            }
        }
    }
    solution
}

fn complement(i: i64) -> i64 {
    2020 - i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 878724);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 201251610);
    }
}
