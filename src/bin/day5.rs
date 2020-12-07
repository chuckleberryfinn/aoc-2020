use aoc::*;

fn main() -> Result<()> {
    let codes = get_codes();
    println!("Day 5 part 1 result is {}", part1(&codes));
    println!("Day 5 part 2 result is {}", part2(&codes));
    Ok(())
}

fn get_codes() -> Vec<String> {
    input("5.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn bsearch(code: &str, start: usize, end: usize) -> usize {
    if code.is_empty() {
        return end;
    }

    match code.chars().next().unwrap() {
        'F' | 'L' => bsearch(&code[1..], start, (start + end) / 2),
        _ => bsearch(&code[1..], (start + end) / 2, end),
    }
}

fn calculate_id(code: &str) -> usize {
    bsearch(&code[..7], 0, 127) * 8 + bsearch(&code[7..], 0, 7)
}

fn part1(codes: &[String]) -> usize {
    codes.iter().map(|c| calculate_id(&c)).max().unwrap()
}

fn part2(codes: &[String]) -> usize {
    let mut all_codes: Vec<usize> = codes.iter().map(|c| calculate_id(&c)).collect();
    all_codes.sort_unstable();
    let start = all_codes[0];
    let end = all_codes.last().unwrap();

    (start..=*end).sum::<usize>() - all_codes.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(&get_codes()) == 864);
    }

    #[test]
    fn test_part2() {
        assert!(part2(&get_codes()) == 739);
    }

    #[test]
    fn test_decode() {
        assert!(calculate_id("FBFBBFFRLR") == 357);
        assert!(calculate_id("BFFFBBFRRR") == 567);
        assert!(calculate_id("FFFBBBFRRR") == 119);
        assert!(calculate_id("BBFFBBFRLL") == 820);
    }
}
