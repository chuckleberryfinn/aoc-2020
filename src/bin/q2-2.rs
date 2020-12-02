use aoc::*;

fn main() -> Result<()> {
    let result = run();
    println!("Day 2 part 2 result is {}", result);
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
        let (min, max, letter, password): (usize, usize, char, &str) = (
            x[0].parse().unwrap(),
            x[1].parse().unwrap(),
            x[2].chars().next().unwrap(),
            x[4],
        );

        if (password.chars().nth(min - 1).unwrap() == letter)
            ^ (password.chars().nth(max - 1).unwrap() == letter)
        {
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
        assert!(run() == 530);
    }
}
