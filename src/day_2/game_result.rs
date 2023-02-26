use std::collections::HashMap;
use std::str::FromStr;

use crate::day_2::enemy_move::EnemyMove;
use crate::day_2::parse_err::ParseErr;
use crate::day_2::player_move::PlayerMove;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn new(player: &PlayerMove, enemy: &EnemyMove) -> Self {
        let results = HashMap::from([
            ((PlayerMove::Rock, EnemyMove::Rock), GameResult::Draw),
            ((PlayerMove::Rock, EnemyMove::Paper), GameResult::Lose),
            ((PlayerMove::Rock, EnemyMove::Scissors), GameResult::Win),
            ((PlayerMove::Paper, EnemyMove::Rock), GameResult::Win),
            ((PlayerMove::Paper, EnemyMove::Paper), GameResult::Draw),
            ((PlayerMove::Paper, EnemyMove::Scissors), GameResult::Lose),
            ((PlayerMove::Scissors, EnemyMove::Rock), GameResult::Lose),
            ((PlayerMove::Scissors, EnemyMove::Paper), GameResult::Win),
            (
                (PlayerMove::Scissors, EnemyMove::Scissors),
                GameResult::Draw,
            ),
        ]);
        *results.get(&(player.clone(), enemy.clone())).unwrap()
    }

    pub fn get_points(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }
}

impl FromStr for GameResult {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(ParseErr),
        }
    }
}
