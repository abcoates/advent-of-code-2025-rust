use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 1)?;

    let part1 = solve_part1(&input)?;
    let part2a = solve_part2a(&input)?;
    let part2b = solve_part2b(&input)?;

    println!("Day 1 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2a: {}", part2a);
    println!("Part 2b: {}", part2b);

    Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut zero_count = 0;
    let mut dial_position = 50;
    for line in input.lines() {
        let direction = &line[0..1];
        let amount = line[1..].parse::<i32>()?;
        if direction == "L" {
            dial_position -= amount;
            while dial_position < 0 { dial_position += 100; }
        } else if direction == "R" {
            dial_position += amount;
            while dial_position > 99 { dial_position -= 100; }
        }
        if dial_position == 0 { zero_count += 1; }
    }
    Ok(zero_count)
}

// Brute force solution
fn solve_part2a(input: &str) -> Result<impl std::fmt::Display> {
    let mut zero_count = 0;
    let mut dial_position = 50;
    for line in input.lines() {
        let direction = &line[0..1];
        let amount = line[1..].parse::<i32>()?;
        if direction == "L" {
            let mut remaining = amount;
            while remaining > 0 {
                remaining -= 1;
                dial_position -= 1;
                if dial_position == 0 {
                    zero_count += 1;
                }
                if dial_position < 0 {
                    dial_position += 100;
                }
            }
        } else if direction == "R" {
            let mut remaining = amount;
            while remaining > 0 {
                remaining -= 1;
                dial_position += 1;
                if dial_position >= 100 {
                    dial_position -= 100;
                }
                if dial_position == 0 {
                    zero_count += 1;
                }
            }
        }
    }
    Ok(zero_count) // 6668 is too high
}

// More thoughtful solution
fn solve_part2b(input: &str) -> Result<impl std::fmt::Display> {
    let mut zero_count = 0;
    let mut dial_position = 50;
    for line in input.lines() {
        let direction = &line[0..1];
        let mut amount = line[1..].parse::<i32>()?;
        if amount >= 100 { // account for complete revolutions of the dial
            zero_count += amount / 100;
            amount = amount % 100;
        }
        if direction == "L" {
            if dial_position > 0 && amount >= dial_position {
                zero_count += 1;
            }
            dial_position -= amount;
            while dial_position < 0 { dial_position += 100; }
        } else if direction == "R" {
            if amount >= 100 - dial_position {
                zero_count += 1;
            }
            dial_position += amount;
            while dial_position > 99 { dial_position -= 100; }
        }
    }
    Ok(zero_count) // 6668 is too high
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "1064");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2b(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "6122");
    }
}
