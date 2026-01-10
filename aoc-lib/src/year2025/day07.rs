use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 7)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 7 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut split_total = 0;
    let start_line = lines.remove(0);
    let max_pos = start_line.len();
    let mut streams: HashSet<usize> = vec!(start_line.find('S').unwrap()).into_iter().collect::<HashSet<usize>>();
    for line in lines {
        let splitters: HashSet<usize> = line.match_indices('^').map(|(index, _)| index).collect::<HashSet<usize>>();
        let splits = splitters.intersection(&streams).into_iter().map(|p| *p).collect::<HashSet<usize>>();
        split_total += splits.len();
        for split in splits {
            streams.remove(&split);
            if split > 0 { streams.insert(split-1); }
            if split < max_pos { streams.insert(split+1); }
        }
    }
    Ok(split_total)
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut timelines_count_map: HashMap<usize, u64> = HashMap::new();
    let start_line = lines.remove(0);
    let max_pos = start_line.len();
    let start_pos = start_line.find('S').unwrap();
    let mut streams: HashSet<usize> = vec!(start_pos).into_iter().collect::<HashSet<usize>>();
    timelines_count_map.insert(start_pos, 1u64);
    for line in lines {
        let splitters: HashSet<usize> = line.match_indices('^').map(|(index, _)| index).collect::<HashSet<usize>>();
        let splits = splitters.intersection(&streams).into_iter().map(|p| *p).collect::<HashSet<usize>>();
        for split in splits {
            let timelines_count_map_copy = timelines_count_map.clone();
            streams.remove(&split);
            let split_timelines_count = timelines_count_map_copy.get(&split).unwrap();
            timelines_count_map.remove(&split);
            if split > 0 {
                streams.insert(split-1);
                if !timelines_count_map.contains_key(&(split-1)) {
                    timelines_count_map.insert(split-1, 0u64);
                }
                timelines_count_map.insert(split-1, timelines_count_map.get(&(split-1)).unwrap() + *split_timelines_count);
            }
            if split < max_pos {
                streams.insert(split+1);
                if !timelines_count_map.contains_key(&(split+1)) {
                    timelines_count_map.insert(split+1, 0u64);
                }
                timelines_count_map.insert(split+1, timelines_count_map.get(&(split+1)).unwrap() + *split_timelines_count);
            }
        }
    }
    let result: u64 = timelines_count_map.values().sum();
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "1711");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "36706966158365");
    }
}
