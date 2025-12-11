use std::collections::HashSet;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(range_str: &str) -> Range {
        let mut end_points = range_str.split('-');
        let start = end_points.next().unwrap().parse::<u64>();
        let end = end_points.next().unwrap().parse::<u64>();
        Range{start: start.unwrap(), end: end.unwrap()}
    }
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let line = input.lines().next().unwrap();
    let ranges: Vec<Range> = line.split(',').map(|s| Range::new(s)).collect();
    let mut result = 0;
    for r in &ranges {
        for i in r.start..=r.end {
            let istr = i.to_string();
            let ilen = istr.len();
            if ilen % 2 == 0 { // if the string version of the number has an even length, and can be split into two halves
                let ihalf = ilen / 2;
                let i1= &istr[0..ihalf];
                let i2= &istr[ihalf..];
                if i1 == i2 {
                    result += i;
                }
            }
        }
    }
    Ok(result)
}

fn repeats(input: &str, prefix: &str) -> bool {
    if input.len() < 1 {
        return true;
    } else if input.starts_with(prefix) {
        return repeats(&input[prefix.len()..], prefix);
    }
    return false;
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let line = input.lines().next().unwrap();
    let ranges: Vec<Range> = line.split(',').map(|s| Range::new(s)).collect();
    let mut result_set = HashSet::new();
    for r in &ranges {
        for i in r.start..=r.end {
            let istr = i.to_string();
            let ilen = istr.len();
            let ihalf = ilen / 2;
            for prefix_len in 1..=ihalf {
                if ilen % prefix_len == 0 { // if prefix fits into istr an even number of times
                    let prefix = &istr[0..prefix_len];
                    if repeats(&istr, prefix) {
                        result_set.insert(i);
                    }
                }
            }
        }
    }
    Ok(result_set.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "17077011375");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "36037497037");
    }
}
