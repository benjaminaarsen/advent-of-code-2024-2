use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let matches = re
        .captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            (a, b)
        })
        .collect::<Vec<(i32, i32)>>();
    let total: i32 = matches.iter().map(|(a, b)| a * b).sum();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total: i32 = re.captures_iter(input)
        .filter_map(|cap| {
            let match_index = cap.get(0).unwrap().start();
            let prefix = &input[..match_index];
            let last_do = prefix.rfind("do()");
            let last_dont = prefix.rfind("don't()");
            let last_pre_operation = match (last_do, last_dont) {
                (Some(a), Some(b)) => {
                    if a > b {
                        Some("do()")
                    } else {
                        Some("don't()")
                    }
                }
                (Some(_), None) => Some("do()"),
                (None, Some(_)) => Some("don't()"),
                (None, None) => None,
            };
            if last_pre_operation == Some("don't()") {
                None
            } else {
                let a = cap[1].parse::<i32>().unwrap();
                let b = cap[2].parse::<i32>().unwrap();
                Some(a * b)
            }
        })
        .sum();
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
