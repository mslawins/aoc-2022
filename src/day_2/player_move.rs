use std::collections::HashMap;
use std::str::FromStr;

use crate::day_2::enemy_move::EnemyMove;
use crate::day_2::game_result::GameResult;
use crate::day_2::parse_err::ParseErr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

impl PlayerMove {
    pub fn new(enemy: EnemyMove, result: GameResult) -> PlayerMove {
        let results = HashMap::from([
            ((EnemyMove::Rock, GameResult::Win), PlayerMove::Paper),
            ((EnemyMove::Rock, GameResult::Draw), PlayerMove::Rock),
            ((EnemyMove::Rock, GameResult::Lose), PlayerMove::Scissors),
            ((EnemyMove::Paper, GameResult::Win), PlayerMove::Scissors),
            ((EnemyMove::Paper, GameResult::Draw), PlayerMove::Paper),
            ((EnemyMove::Paper, GameResult::Lose), PlayerMove::Rock),
            ((EnemyMove::Scissors, GameResult::Win), PlayerMove::Rock),
            (
                (EnemyMove::Scissors, GameResult::Draw),
                PlayerMove::Scissors,
            ),
            ((EnemyMove::Scissors, GameResult::Lose), PlayerMove::Paper),
        ]);

        *results.get(&(enemy, result)).unwrap()
    }

    pub fn get_points(&self) -> i32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissors => 3,
        }
    }
}

impl FromStr for PlayerMove {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlayerMove::Rock),
            "Y" => Ok(PlayerMove::Paper),
            "Z" => Ok(PlayerMove::Scissors),
            _ => Err(ParseErr),
        }
    }
}
