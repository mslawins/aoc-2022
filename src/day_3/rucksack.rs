use std::str::FromStr;

pub fn get_points_from_letter(key: char) -> usize {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    alphabet.iter().position(|letter| *letter == key).unwrap() + 1
}

pub fn get_intersection(vectors: Vec<Vec<char>>) -> char {
    vectors
        .into_iter()
        .map(|v| v.into_iter().collect::<std::collections::HashSet<_>>())
        .reduce(|acc, next| acc.intersection(&next).copied().collect())
        .and_then(|intersection| intersection.into_iter().next())
        .unwrap()
}

pub struct Rucksack {
    value: String,
}

impl Rucksack {
    pub fn get_points(&self) -> usize {
        let (left, right) = self.value.split_at(self.value.len() / 2);
        let left: Vec<char> = left.chars().collect();
        let right: Vec<char> = right.chars().collect();
        let intersection = get_intersection(vec![left, right]);
        get_points_from_letter(intersection)
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack {
            value: s.to_owned(),
        })
    }
}
