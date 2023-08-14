use self::{stack_move::StackMove, supply_stacks::SupplyStacks};
use std::str::FromStr;

mod stack_move;
mod supply_stacks;

pub fn day_5() {
    let input =
        std::fs::read_to_string("./src/inputs/5.txt").expect("File not found! Run in project root");
    let task_one = task_one(&input);
    let task_two = task_two(&input);

    println!("Day 5: task one: {}, task two: {}", task_one, task_two);
}

fn task_one(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut stacks = SupplyStacks::from_str(parts.first().unwrap()).unwrap();
    let moves: Vec<StackMove> = parts
        .get(1)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| StackMove::from_str(l).unwrap())
        .collect();

    stacks.apply_moves_by_one(moves);
    stacks.get_top_crates()
}

fn task_two(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut stacks = SupplyStacks::from_str(parts.first().unwrap()).unwrap();
    let moves: Vec<StackMove> = parts
        .get(1)
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| StackMove::from_str(l).unwrap())
        .collect();

    stacks.bulk_apply_moves(moves);
    stacks.get_top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./day_5/test_input.txt");

    #[test]
    fn test_task_one() {
        let result = task_one(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_task_two() {
        let result = task_two(INPUT);
        assert_eq!(result, "MCD");
    }
}
