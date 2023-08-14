use std::str::FromStr;

use super::stack_move::StackMove;

#[derive(Debug, PartialEq)]
pub struct SupplyStacks {
    stacks: Vec<Vec<char>>,
}

impl SupplyStacks {
    pub fn get_top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }

    pub fn apply_moves_by_one(&mut self, stack_moves: Vec<StackMove>) {
        for stack_move in stack_moves {
            for _ in 0..stack_move.times {
                self.apply_move(stack_move.from, stack_move.to);
            }
        }
    }

    pub fn bulk_apply_moves(&mut self, stack_moves: Vec<StackMove>) {
        for stack_move in stack_moves {
            self.bulk_apply_move(stack_move);
        }
    }

    fn apply_move(&mut self, from: usize, to: usize) {
        let target = self.stacks.get_mut(from - 1).unwrap().pop().unwrap();
        self.stacks.get_mut(to - 1).unwrap().push(target);
    }

    fn bulk_apply_move(&mut self, stack_move: StackMove) {
        let from_vec = self.stacks.get_mut(stack_move.from - 1).unwrap();
        let tail = from_vec.split_off(from_vec.len() - stack_move.times);
        let to_vec = self.stacks.get_mut(stack_move.to - 1).unwrap();
        to_vec.extend(tail)
    }
}

impl FromStr for SupplyStacks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<_> = s.split('\n').collect();
        let last = lines.pop().unwrap();
        let indexes = get_crates_indexes(last);

        let mut stacks: Vec<Vec<char>> = vec![vec![]; indexes.len()];

        for (index, value) in indexes.iter().enumerate() {
            let inner_vec = stacks.get_mut(index).unwrap();

            for line in lines.iter().rev() {
                let char = line.chars().nth(*value).unwrap();

                if char.is_ascii() && !char.is_whitespace() {
                    inner_vec.push(char);
                }
            }
        }

        Ok(SupplyStacks { stacks })
    }
}

fn get_crates_indexes(last: &str) -> Vec<usize> {
    let mut indexes: Vec<usize> = vec![];

    for (index, char) in last.chars().enumerate() {
        if char.is_ascii() && !char.is_whitespace() {
            indexes.push(index);
        }
    }

    indexes
}

#[test]
fn test_from_str_for_supply_stack() {
    let expected = SupplyStacks {
        stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
    };
    let input = include_str!("./test_input.txt");
    let stacks: Vec<&str> = input.split("\n\n").collect();
    let result = SupplyStacks::from_str(stacks.get(0).unwrap()).unwrap();
    assert_eq!(result, expected);
}
