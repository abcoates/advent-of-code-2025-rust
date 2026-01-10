pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub const DAYS: &[(&str, fn() -> anyhow::Result<()>)] = &[
    ("1", day01::solve),
    ("2", day02::solve),
    ("3", day03::solve),
    ("4", day04::solve),
    ("5", day05::solve),
    ("6", day06::solve),
    ("7", day07::solve),
];
