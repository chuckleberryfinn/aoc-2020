use aoc::*;

fn main() -> Result<()> {
    println!("Day 2 part 1 result is {}", part1());
    println!("Day 2 part 2 result is {}", part2());
    Ok(())
}

fn part1() -> usize {
    count_valid_passwords(valid_1)
}

fn part2() -> usize {
    count_valid_passwords(valid_2)
}

fn count_valid_passwords(valid: fn((usize, usize, char, &str)) -> bool) -> usize {
    input("2.txt")
        .unwrap()
        .lines()
        .map(|s| parse_line(s))
        .filter(|p| valid(*p))
        .count()
}

fn parse_line(p: &str) -> (usize, usize, char, &str) {
    let x: Vec<&str> = p.split(|c| c == '-' || c == ' ' || c == ':').collect();
    (
        x[0].parse().unwrap(),
        x[1].parse().unwrap(),
        x[2].chars().next().unwrap(),
        x[4],
    )
}

fn valid_1((min, max, letter, password): (usize, usize, char, &str)) -> bool {
    let count = password.matches(letter).count();
    (min..=max).contains(&count)
}

fn valid_2((min, max, letter, password): (usize, usize, char, &str)) -> bool {
    let chars: Vec<char> = password.chars().collect();
    (chars[min - 1] == letter) ^ (chars[max - 1] == letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1() == 660);
    }

    #[test]
    fn test_part2() {
        assert!(part2() == 530);
    }
}
