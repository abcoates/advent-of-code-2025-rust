use std::fmt;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 5)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 5 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Clone)]
struct NumRange {
    start: u64,
    end: u64
}

impl NumRange {
    fn contains(&self, x: u64) -> bool {
        self.start <= x && x <= self.end
    }
    fn size(&self) -> u64 {
        self.end - self.start + 1
    }
    fn join(&self, other: &NumRange) -> Option<NumRange> {
        if self.end+1 < other.start || other.end+1 < self.start { // allow adjacent, non-overlapping ranges to be joined
            None
        } else {
            Some(NumRange{start: self.start.min(other.start), end: self.end.max(other.end)})
        }
    }
}

impl fmt::Display for NumRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut ranges: Vec<NumRange> = Vec::new();
    let mut fresh_count = 0u16;
    let mut found_blank_line = false;
    for line in input.lines() {
        if line.len() == 0 {
            found_blank_line = true;
        } else if found_blank_line {
            let value = line.parse::<u64>()?;
            for range in ranges.clone() {
                if range.contains(value) {
                    fresh_count += 1;
                    break; // only count the value once
                }
            }
        } else {
            let mut endpoints = line.split("-");
            let start = endpoints.next().unwrap().parse::<u64>()?;
            let end = endpoints.next().unwrap().parse::<u64>()?;
            let range = NumRange{start:start, end:end};
            ranges.push(range);
        }
    }
    Ok(fresh_count)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut ranges: Vec<NumRange> = Vec::new();
    let mut fresh_count = 0u64;
    for line in input.lines() {
        if line.len() == 0 {
            break;
        } else {
            let mut endpoints = line.split("-");
            let start = endpoints.next().unwrap().parse::<u64>()?;
            let end = endpoints.next().unwrap().parse::<u64>()?;
            let range = NumRange{start:start, end:end};
            ranges.push(range);
        }
    }
    ranges.sort_by_key(|r| (r.start, r.end));

    // Join the ranges into as few as possible
    let mut joined_ranges: Vec<NumRange> = Vec::new();
    for range in ranges {
        if joined_ranges.len() == 0 {
            joined_ranges.push(range);
        } else {
            let mut is_joined = false; // whether 'range' was joined to an existing range
            let mut new_joined_ranges: Vec<NumRange> = Vec::new();
            for joined_range in joined_ranges.clone() {
                let new_range = joined_range.join(&range);
                match new_range {
                    Some(new_joined_range) => {
                        is_joined = true;
                        new_joined_ranges.push(new_joined_range);
                    }
                    None => {
                        new_joined_ranges.push(joined_range);
                    }
                }
            }
            joined_ranges = new_joined_ranges;
            if !is_joined {
                joined_ranges.push(range);
            }
        }
    }
    joined_ranges.sort_by_key(|r| r.start);

    // Now sum the sizes of the joined ranges.
    for range in joined_ranges {
        fresh_count += range.size();
    }

    Ok(fresh_count) // 348748964797164 is too high
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "613");
    }

    #[test]
    fn test_part2b() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "336495597913098");
    }
}
