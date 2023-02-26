mod enemy_move;
mod game_result;
mod parse_err;
mod player_move;
mod single_game;

use crate::day_2::single_game::SingleGame;

pub fn day_2() {
    let input =
        std::fs::read_to_string("./src/inputs/2.txt").expect("File not found! Run in project root");
    let task_one = task_one(&input);
    let task_two = task_two(&input);

    println!("Day 2: task one: {}, task two: {}", task_one, task_two);
}

fn task_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    lines
        .into_iter()
        .map(|s: &str| SingleGame::from_ruleset_one(s).unwrap().get_points())
        .sum()
}

fn task_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();
    lines
        .into_iter()
        .map(|s: &str| SingleGame::from_ruleset_two(s).unwrap().get_points())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_task_one() {
        let result = task_one(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_task_two() {
        let result = task_two(INPUT);
        assert_eq!(result, 12);
    }
}
