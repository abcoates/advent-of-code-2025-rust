use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 4)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 4 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut roll_count = 0u16;
    let char_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = char_matrix.len();
    let width = char_matrix[0].len();
    let ymax = height - 1;
    let xmax = width - 1;
    for y in 0..height {
        for x in 0..width {
            let ch = char_matrix[y][x];
            if ch == '@' {
                let mut adjacent_roll_count = 0u16;
                if x > 0 && char_matrix[y][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && char_matrix[y][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if y > 0 && char_matrix[y-1][x] == '@' {
                    adjacent_roll_count += 1;
                }
                if y < ymax && char_matrix[y+1][x] == '@' {
                    adjacent_roll_count += 1;
                }
                if x > 0 && y > 0 && char_matrix[y-1][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x > 0 && y < ymax && char_matrix[y+1][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && y > 0 && char_matrix[y-1][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && y < ymax && char_matrix[y+1][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if adjacent_roll_count < 4 {
                    roll_count += 1;
                }
            }
        }
    }
    Ok(roll_count)
}

fn remove_rolls(char_matrix: &mut Vec<Vec<char>>, height: usize, width: usize) -> u16 {
    let mut removed_roll_count = 0u16;
    let ymax = height - 1;
    let xmax = width - 1;
    for y in 0..height {
        for x in 0..width {
            let ch = char_matrix[y][x];
            if ch == '@' {
                let mut adjacent_roll_count = 0u16;
                if x > 0 && char_matrix[y][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && char_matrix[y][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if y > 0 && char_matrix[y-1][x] == '@' {
                    adjacent_roll_count += 1;
                }
                if y < ymax && char_matrix[y+1][x] == '@' {
                    adjacent_roll_count += 1;
                }
                if x > 0 && y > 0 && char_matrix[y-1][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x > 0 && y < ymax && char_matrix[y+1][x-1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && y > 0 && char_matrix[y-1][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if x < xmax && y < ymax && char_matrix[y+1][x+1] == '@' {
                    adjacent_roll_count += 1;
                }
                if adjacent_roll_count < 4 {
                    removed_roll_count += 1;
                    char_matrix[y][x] = 'x';
                }
            }
        }
    }
    removed_roll_count
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut char_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = char_matrix.len();
    let width = char_matrix[0].len();
    let mut total_removed_rolls = 0u16;
    let mut removed_rolls = 1u16;
    while removed_rolls > 0 {
        removed_rolls = remove_rolls(&mut char_matrix, height, width);
        total_removed_rolls += removed_rolls;
    }
    Ok(total_removed_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "1397");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "8758");
    }
}
