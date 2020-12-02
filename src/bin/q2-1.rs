use aoc::*;

fn main() -> Result<()> {
    let result = run();
    println!("Day 2 part 1 result is {}", result);
    Ok(())
}

fn run() -> i64 {
    let passwords: Vec<String> = input("2.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut valid = 0;
    for p in passwords {
        let x: Vec<&str> = p.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let (min, max, letter, password): (usize, usize, &str, &str) =
            (x[0].parse().unwrap(), x[1].parse().unwrap(), x[2], x[4]);
        let count = password.matches(letter).count();

        if min <= count && count <= max {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert!(run() == 660);
    }
}
