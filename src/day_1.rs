pub fn day_1() {
    let input =
        std::fs::read_to_string("./src/inputs/1.txt").expect("File not found! Run in project root");
    let task_one = task_one(&input);
    let task_two = task_two(&input);

    println!("Day 1: task one: {}, task two: {}", task_one, task_two);
}

fn task_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|single_load| {
            single_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn task_two(input: &str) -> u32 {
    let mut total_loads: Vec<u32> = input
        .split("\n\n")
        .map(|single_load| {
            single_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    total_loads.sort_unstable();
    total_loads[total_loads.len() - 3..].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_task_one() {
        let result = task_one(INPUT);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_task_two() {
        let result = task_two(INPUT);
        assert_eq!(result, 45000);
    }
}
