use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct AssignmentPair {
    first_from: u32,
    first_to: u32,
    second_from: u32,
    second_to: u32,
}

impl AssignmentPair {
    pub fn contains(&self) -> bool {
        let is_second_in_first =
            self.second_from >= self.first_from && self.second_to <= self.first_to;
        let is_first_in_second =
            self.second_from <= self.first_from && self.second_to >= self.first_to;
        is_first_in_second || is_second_in_first
    }

    pub fn overlaps(&self) -> bool {
        !(self.second_from > self.first_to || self.first_from > self.second_to)
    }
}

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .split(|c| c == ',' || c == '-')
            .map(|c| c.parse().unwrap())
            .collect();

        Ok(AssignmentPair {
            first_from: *parts.first().unwrap(),
            first_to: *parts.get(1).unwrap(),
            second_from: *parts.get(2).unwrap(),
            second_to: *parts.get(3).unwrap(),
        })
    }
}

#[test]
fn test_assignment_pair_from_str() {
    let str = "1-2,3-4";
    let result = AssignmentPair::from_str(str).unwrap();
    let expected = AssignmentPair {
        first_from: 1,
        first_to: 2,
        second_from: 3,
        second_to: 4,
    };
    assert_eq!(expected, result);
}
