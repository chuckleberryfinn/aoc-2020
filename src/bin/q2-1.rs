use aoc::*;

fn main() -> Result<()> {
    let result = run();
    println!("Day 2 part 1 result is {}", result);
    Ok(())
}

fn run() -> usize {
    let passwords: Vec<String> = input("2.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    passwords
        .iter()
        .map(|p| valid_password(&p))
        .filter(|r| *r)
        .count()
}

fn valid_password(p: &str) -> bool {
    let x: Vec<&str> = p.split(|c| c == '-' || c == ' ' || c == ':').collect();
    let (min, max, letter, password): (usize, usize, char, &str) = (
        x[0].parse().unwrap(),
        x[1].parse().unwrap(),
        x[2].chars().next().unwrap(),
        x[4],
    );

    let count = password.matches(letter).count();
    min <= count && count <= max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert!(run() == 660);
    }
}
