use aoc::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let result = run();
    println!("Day 1 part 2 result is {}", result);
    Ok(())
}

fn run() -> i64 {
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
    fn test_run() {
        assert!(run() == 201251610);
    }
}
