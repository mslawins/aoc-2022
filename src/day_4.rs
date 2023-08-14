mod assignment_pair;

use assignment_pair::AssignmentPair;
use std::str::FromStr;

pub fn day_4() {
    let input =
        std::fs::read_to_string("./src/inputs/4.txt").expect("File not found! Run in project root");
    let task_one = task_one(&input);
    let task_two = task_two(&input);

    println!("Day 4: task one: {}, task two: {}", task_one, task_two);
}

fn task_one(input: &str) -> usize {
    let pairs: Vec<AssignmentPair> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| AssignmentPair::from_str(s).unwrap())
        .filter(|pair| pair.contains())
        .collect();
    pairs.len()
}

fn task_two(input: &str) -> usize {
    let pairs: Vec<AssignmentPair> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| AssignmentPair::from_str(s).unwrap())
        .filter(|pair| pair.overlaps())
        .collect();
    pairs.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_task_one() {
        let result = task_one(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_task_two() {
        let result = task_two(INPUT);
        assert_eq!(result, 4);
    }
}
