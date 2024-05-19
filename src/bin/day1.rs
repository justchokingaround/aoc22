use anyhow::Result;
use aoc::{
    parse_file,
    Part::{self, *},
};

const PATH: &str = "./data/1.input";

fn calculate_max(part: Part) -> Result<u32> {
    let lines = parse_file::<String>(&PATH)?;
    let mut maxs: Vec<u32> = lines
        .split(|line| line.is_empty())
        .filter_map(|group| group.iter().map(|n| n.parse::<u32>().ok()).sum())
        .collect();
    Ok(match part {
        Part1 => maxs.iter().max().unwrap_or(&0).clone(),
        Part2 => {
            maxs.sort_unstable();
            maxs.iter().rev().take(3).sum()
        }
    })
}

fn main() {
    println!("Part 1 - {:?}", calculate_max(Part1).unwrap());
    println!("Part 2 - {:?}", calculate_max(Part2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1: u32 = 71934;
        let result_2: u32 = 211447;
        assert_eq!(result_1, calculate_max(Part1).unwrap());
        assert_eq!(result_2, calculate_max(Part2).unwrap());
    }
}
