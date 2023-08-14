mod rucksack;

use self::rucksack::{get_intersection, get_points_from_letter, Rucksack};
use std::str::FromStr;

pub fn day_3() {
    let input =
        std::fs::read_to_string("./src/inputs/3.txt").expect("File not found! Run in project root");
    let task_one = task_one(&input);
    let task_two = task_two(&input);

    println!("Day 3: task one: {}, task two: {}", task_one, task_two);
}

fn task_one(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    let rucksacks: Vec<Rucksack> = lines
        .into_iter()
        .map(|line| Rucksack::from_str(line).unwrap())
        .collect();
    rucksacks
        .into_iter()
        .map(|rucksack| rucksack.get_points())
        .sum()
}

fn task_two(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let chunks: Vec<Vec<Vec<char>>> = lines.chunks(3).map(|part| part.to_vec()).collect();

    chunks
        .into_iter()
        .map(get_intersection)
        .map(get_points_from_letter)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_task_one() {
        let result = task_one(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_task_two() {
        let result = task_two(INPUT);
        assert_eq!(result, 70);
    }
}
