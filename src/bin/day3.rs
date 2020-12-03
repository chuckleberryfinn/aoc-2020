use aoc::*;

fn main() -> Result<()> {
    let grid: Vec<String> = get_grid();
    let result = part1(&grid);
    println!("Day 3 part 1 result is {}", result);
    let result = part2(&grid);
    println!("Day 3 part 2 result is {}", result);
    Ok(())
}

fn get_grid() -> Vec<String> {
    input("3.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn part1(grid: &[String]) -> usize {
    slope((3, 1), &grid)
}

fn part2(grid: &[String]) -> usize {
    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    directions.iter().map(|d| slope(*d, &grid)).product()
}

fn slope((x, y): (usize, usize), grid: &[String]) -> usize {
    grid.iter()
        .step_by(y)
        .enumerate()
        .skip(1)
        .filter(|(i, r)| r.chars().nth((i * x) % r.len()).unwrap() == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid: Vec<String> = get_grid();
        assert!(part1(&grid) == 178);
    }

    #[test]
    fn test_part2() {
        let grid: Vec<String> = get_grid();
        assert!(part2(&grid) == 3492520200);
    }
}
