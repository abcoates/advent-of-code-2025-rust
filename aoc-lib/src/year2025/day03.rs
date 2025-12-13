use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 3)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 3 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut joltage = 0u32;
    for bank in input.lines() {
        let digits: Vec<char> = bank.chars().collect();
        let digit1 = digits[0..(digits.len()-1)].iter().max().unwrap();
        let location1 = digits.iter().position(|x| digit1.eq(x)).unwrap();
        let digit2 = digits[(location1+1)..].iter().max().unwrap();
        let bank_joltage = format!("{}{}", digit1, digit2).parse::<u32>()?;
        joltage += bank_joltage;
    }
    Ok(joltage)
}

fn largest_digit_string(bank: &str, digit_count: usize) -> String {
    if digit_count == 0 {
        return "".to_owned()
    }
    let digits: Vec<char> = bank.chars().collect();
    if bank.len() <= digit_count {
        bank.to_owned()
    } else {
        let free_digit_count = bank.len() - digit_count + 1;
        let max_digit = digits[0..free_digit_count].iter().max().unwrap();
        let max_location = digits.iter().position(|x| max_digit.eq(x)).unwrap();
        format!("{}{}", max_digit, largest_digit_string(&bank[(max_location+1)..], digit_count-1))
    }
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut joltage = 0u64;
    for bank in input.lines() {
        let largest = largest_digit_string(&bank, 12);
        let bank_joltage = largest.parse::<u64>()?;
        joltage += bank_joltage;
    }
    Ok(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "17179");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "170025781683941");
    }
}
