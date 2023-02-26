use std::str::FromStr;

use crate::day_2::parse_err::ParseErr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EnemyMove {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for EnemyMove {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(EnemyMove::Rock),
            "B" => Ok(EnemyMove::Paper),
            "C" => Ok(EnemyMove::Scissors),
            _ => Err(ParseErr),
        }
    }
}
