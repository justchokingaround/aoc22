use anyhow::Result;
use aoc::{
    parse_file,
    Part::{self, *},
};

const PATH: &str = "./data/3.input";

fn char_to_u32(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn solution(part: Part) -> Result<u32> {
    let content = parse_file::<String>(&PATH)?;
    let mut sum = 0;
    match part {
        Part1 => content.iter().for_each(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let common_char = first_half
                .chars()
                .find(|c| second_half.contains(*c))
                .unwrap();
            sum += char_to_u32(common_char);
        }),
        Part2 => {
            content.chunks(3).for_each(|group| {
                let common_char = group
                    .first()
                    .unwrap()
                    .chars()
                    .find(|c| {
                        group.get(1).unwrap().contains(*c) && group.get(2).unwrap().contains(*c)
                    })
                    .unwrap();
                sum += char_to_u32(common_char);
            });
        }
    }
    Ok(sum)
}

fn main() {
    println!("Part 1 - {:?}", solution(Part1).unwrap());
    println!("Part 2 - {:?}", solution(Part2).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_1: u32 = 7674;
        let result_2: u32 = 2805;
        assert_eq!(result_1, solution(Part1).unwrap());
        assert_eq!(result_2, solution(Part2).unwrap());
    }
}
