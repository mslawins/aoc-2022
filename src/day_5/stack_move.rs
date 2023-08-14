use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct StackMove {
    pub from: usize,
    pub to: usize,
    pub times: usize,
}

impl FromStr for StackMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        Ok(StackMove {
            times: parts.get(1).unwrap().parse().unwrap(),
            from: parts.get(3).unwrap().parse().unwrap(),
            to: parts.get(5).unwrap().parse().unwrap(),
        })
    }
}

#[test]
fn test_from_str_for_move() {
    let str = "move 8 from 6 to 4";
    let expected = StackMove {
        from: 6,
        to: 4,
        times: 8,
    };
    let result = StackMove::from_str(str).unwrap();
    assert_eq!(expected, result);
}
