use aoc::*;

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
    let mut solution = -1;

    for i in 0..expenses.len() {
        for j in (i + 1)..expenses.len() {
            for k in (j + 1)..expenses.len() {
                let (x, y, z) = (expenses[i], expenses[j], expenses[k]);
                if (x + y + z) == 2020 {
                    solution = x * y * z;
                    break;
                }
            }
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert!(run() == 201251610);
    }
}
