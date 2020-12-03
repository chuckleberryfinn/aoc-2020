use aoc::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let result = part_1();
    println!("Day 1 part 1 result is {}", result);
    let result = part_2();
    println!("Day 1 part 2 result is {}", result);
    Ok(())
}

fn part_1() -> i64 {
    let expenses: HashSet<i64> = input("1.txt")
        .unwrap()
        .lines()
        .map(|e| e.parse().unwrap())
        .collect();

    let complements: HashSet<i64> = expenses.iter().map(|e| complement(*e)).collect();

    expenses.intersection(&complements).product()
}

fn part_2() -> i64 {
    let expenses: Vec<i64> = input("1.txt")
        .unwrap()
        .lines()
        .map(|e| e.parse().unwrap())
        .collect();

    let expenses_hash: HashSet<i64> = expenses.iter().copied().collect();
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
    fn test_part_1() {
        assert!(part_1() == 878724);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 201251610);
    }
}