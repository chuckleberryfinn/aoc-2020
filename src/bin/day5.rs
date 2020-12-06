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

fn bsearch(code: &str, limit: usize) -> usize {
    let mut m = limit;
    let mut p = 0;
    for c in code.chars() {
        match c {
            'F' | 'L' => m = (m + p) / 2,
            'B' | 'R' => p = (m + p) / 2,
            _ => (),
        }
    }
    m
}

fn decode(code: &str) -> (usize, usize) {
    (bsearch(&code[..7], 127), bsearch(&code[7..], 7))
}

fn calculate_id(code: &str) -> usize {
    let row_seat = decode(code);
    row_seat.0 * 8 + row_seat.1
}

fn part1(codes: &[String]) -> usize {
    codes.iter().map(|c| calculate_id(&c)).max().unwrap()
}

fn part2(codes: &[String]) -> usize {
    let all_codes: Vec<usize> = codes.iter().map(|c| calculate_id(&c)).collect();

    let start = all_codes.iter().min().unwrap();
    let end = all_codes.iter().max().unwrap();

    (*start..=*end).sum::<usize>() - all_codes.iter().sum::<usize>()
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
        assert!(decode("FBFBBFFRLR") == (44, 5));
        assert!(decode("BFFFBBFRRR") == (70, 7));
        assert!(decode("FFFBBBFRRR") == (14, 7));
        assert!(decode("BBFFBBFRLL") == (102, 4));
    }
}
