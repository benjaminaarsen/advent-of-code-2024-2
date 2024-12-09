advent_of_code::solution!(2);

fn check_report(report: &Vec<i32>) -> bool {
    let all_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let all_decreasing = report.windows(2).all(|window| window[0] > window[1]);
    if !all_increasing && !all_decreasing {
        return false;
    }
    for window in report.windows(2) {
        if (window[0] - window[1]).abs() < 1 || (window[0] - window[1]).abs() > 3 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    'reports: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let all_increasing = levels.windows(2).all(|window| window[0] < window[1]);
        let all_decreasing = levels.windows(2).all(|window| window[0] > window[1]);
        if !all_increasing && !all_decreasing {
            continue;
        }
        for window in levels.windows(2) {
            if (window[0] - window[1]).abs() < 1 || (window[0] - window[1]).abs() > 3 {
                continue 'reports;
            }
        }
        println!("{} is safe", report);
        safe_count += 1;
    }
    Some(safe_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for report in input.lines() {
        let report: Vec<i32> = report
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();

        if check_report(&report) {
            safe_count += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if check_report(&new_report) {
                safe_count += 1;
                break;
            }
        }
    }
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
