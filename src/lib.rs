use anyhow::Result;
use std::str::FromStr;

pub enum Part {
    Part1,
    Part2,
}

pub fn parse_file<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(&path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
