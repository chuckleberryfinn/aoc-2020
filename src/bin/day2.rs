use aoc::*;

fn main() -> Result<()> {
    let result = part_1();
    println!("Day 2 part 1 result is {}", result);
    let result = part_2();
    println!("Day 2 part 2 result is {}", result);
    Ok(())
}

fn part_1() -> usize {
    input("2.txt")
        .unwrap()
        .lines()
        .map(|s| parse_line(s))
        .filter(|p| valid_password_1(*p))
        .count()
}

fn part_2() -> usize {
    input("2.txt")
        .unwrap()
        .lines()
        .map(|s| parse_line(s))
        .filter(|p| valid_password_2(*p))
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

fn valid_password_1((min, max, letter, password): (usize, usize, char, &str)) -> bool {
    let count = password.matches(letter).count();
    (min..=max).contains(&count)
}

fn valid_password_2((min, max, letter, password): (usize, usize, char, &str)) -> bool {
    let chars: Vec<char> = password.chars().collect();
    (chars[min - 1] == letter) ^ (chars[max - 1] == letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 660);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 530);
    }
}
