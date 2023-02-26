use std::str::FromStr;

use crate::day_2::enemy_move::EnemyMove;
use crate::day_2::game_result::GameResult;
use crate::day_2::parse_err::ParseErr;
use crate::day_2::player_move::PlayerMove;

#[derive(Debug)]
pub struct SingleGame {
    enemy: EnemyMove,
    player: PlayerMove,
}

impl SingleGame {
    pub fn from_ruleset_one(s: &str) -> Result<Self, ParseErr> {
        let split: Vec<&str> = s.split_whitespace().collect();
        let enemy = EnemyMove::from_str(split.get(0).unwrap())?;
        let player = PlayerMove::from_str(split.get(1).unwrap())?;

        Ok(SingleGame { enemy, player })
    }

    pub fn from_ruleset_two(s: &str) -> Result<Self, ParseErr> {
        let split: Vec<&str> = s.split_whitespace().collect();
        let enemy = EnemyMove::from_str(split.get(0).unwrap())?;
        let result = GameResult::from_str(split.get(1).unwrap())?;
        let player = PlayerMove::new(enemy, result);

        Ok(SingleGame { enemy, player })
    }

    pub fn get_points(&self) -> i32 {
        let move_points = self.player.get_points();
        let game = GameResult::new(&self.player, &self.enemy);
        move_points + game.get_points()
    }
}
