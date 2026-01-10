use std::collections::HashSet;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 6)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 6 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn build_space_map(line: &str) -> HashSet<u16> {
    let mut result: HashSet<u16> = HashSet::new();
    let line_len = line.len() as u16;
    let chars: Vec<char> = line.chars().collect();
    for chidx in 0..line_len {
        if chars[chidx as usize] == ' ' {
            result.insert(chidx);
        }
    }
    result
}

fn split_line_at_columns(line: &str, column_breaks: &Vec<u16>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut last_column = 0u16;
    for column_break in column_breaks {
        result.push(line[(last_column as usize)..(*column_break as usize)].to_string());
        last_column = *column_break+1;
    }
    result.push(line[(last_column as usize)..].to_string());
    result
}

fn calc_col(col_vec: Vec<String>) -> u64 {
    let mut rev_vec = col_vec.into_iter().rev().collect::<Vec<String>>();
    let operator_untrimmed = rev_vec.remove(0);
    let operator = operator_untrimmed.trim().to_string();
    if operator == "+" {
        rev_vec.iter().map(|s| s.trim().parse::<u64>().unwrap()).sum()
    } else if operator == "*" {
        rev_vec.iter().map(|s| s.trim().parse::<u64>().unwrap()).product()
    } else {
        0u64
    }
}

fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
    // First we work out which columns only have spaces, i.e. which columns separate different calculations.
    let lines: Vec<&str> = input.lines().collect();
    let space_maps: Vec<HashSet<u16>> = lines.clone().iter().map(|l| build_space_map(l)).collect();
    let mut column_breaks_set: HashSet<u16> = HashSet::new();
    for space_map in space_maps {
        if column_breaks_set.len() == 0 {
            column_breaks_set = space_map;
        } else {
            column_breaks_set = column_breaks_set.intersection(&space_map).cloned().collect();
        }
    }
    let mut column_breaks: Vec<u16> = column_breaks_set.into_iter().collect();
    column_breaks.sort();

    // Now split the lines up into separate vectors of stripped strings
    let split_lines: Vec<Vec<String>> = lines.iter().map(|l| split_line_at_columns(l, &column_breaks)).collect();

    // Pivot the line vector into column vectors
    let col_count = split_lines.iter().map(|lvec| lvec.len()).min().unwrap();
    let mut split_cols: Vec<Vec<String>> = Vec::new();
    for colidx in 0..col_count {
        let mut col_vec: Vec<String> = Vec::new();
        for lvec in split_lines.clone() {
            col_vec.push(lvec[colidx].to_string());
        }
        split_cols.push(col_vec);
    }

    // Calculate each column and sum the result
    let mut result = 0u64;
    for col_vec in split_cols {
        // println!("{}", col_vec.clone().join(", "));
        let col_vec_result = calc_col(col_vec);
        // println!("col vec result: {}", col_vec_result);
        result += col_vec_result;
    }
    Ok(result)
}

fn calc_col2(col_vec: Vec<String>) -> u64 {
    // Now the number strings are vertical, rather than horizontal, as on 'calc_col'.
    let mut rev_vec = col_vec.into_iter().rev().collect::<Vec<String>>();
    let operator_untrimmed = rev_vec.remove(0);
    let operator = operator_untrimmed.trim().to_string();
    rev_vec = rev_vec.clone().into_iter().rev().collect::<Vec<String>>(); // need to parse these unreversed
    let row_length = rev_vec.iter().map(|s| s.len()).max().unwrap();
    let mut vertical_vec: Vec<String> = Vec::new();
    for rowidx in 0..row_length {
        let vert_str = rev_vec.iter().map(|v| v.chars().nth(rowidx).get_or_insert(' ').clone()).collect::<String>().trim().to_string();
        vertical_vec.push(vert_str);
    }
    if operator == "+" {
        vertical_vec.iter().map(|s| s.parse::<u64>().unwrap()).sum()
    } else if operator == "*" {
        vertical_vec.iter().map(|s| s.parse::<u64>().unwrap()).product()
    } else {
        println!("!!!! unrecognised operator: '{}'", operator);
        0u64
    }
}

fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
    // First we work out which columns only have spaces, i.e. which columns separate different calculations.
    let lines: Vec<&str> = input.lines().collect();
    let space_maps: Vec<HashSet<u16>> = lines.clone().iter().map(|l| build_space_map(l)).collect();
    let mut column_breaks_set: HashSet<u16> = HashSet::new();
    for space_map in space_maps {
        if column_breaks_set.len() == 0 {
            column_breaks_set = space_map;
        } else {
            column_breaks_set = column_breaks_set.intersection(&space_map).cloned().collect();
        }
    }
    let mut column_breaks: Vec<u16> = column_breaks_set.into_iter().collect();
    column_breaks.sort();

    // Now split the lines up into separate vectors of stripped strings
    let split_lines: Vec<Vec<String>> = lines.iter().map(|l| split_line_at_columns(l, &column_breaks)).collect();

    // Pivot the line vector into column vectors
    let col_count = split_lines.iter().map(|lvec| lvec.len()).min().unwrap();
    let mut split_cols: Vec<Vec<String>> = Vec::new();
    for colidx in 0..col_count {
        let mut col_vec: Vec<String> = Vec::new();
        for lvec in split_lines.clone() {
            col_vec.push(lvec[colidx].to_string());
        }
        split_cols.push(col_vec);
    }

    // Calculate each column and sum the result
    let mut result = 0u64;
    for col_vec in split_cols {
        // println!("{}", col_vec.clone().join(", "));
        let col_vec_result = calc_col2(col_vec);
        // println!("col vec result: {}", col_vec_result);
        result += col_vec_result;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "6417439773370");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "11044319475191");
    }
}
