use itertools::Itertools;

advent_of_code::solution!(7);

fn solve(operations: &[char], equations: Vec<(u64, Vec<u64>)>) -> u64 {
    let mut correct_equations_sum = 0;
    for equation in equations.iter() {
        let repeat = equation.1.len(); // Number of repetitions

        let combinations = std::iter::repeat(operations)
            .take(repeat - 1)
            .multi_cartesian_product();

        for combo in combinations {
            let mut result = equation.1[0];

            for (i, &op) in combo.iter().enumerate() {
                match op {
                    '*' => result *= equation.1[i + 1],
                    '+' => result += equation.1[i + 1],
                    '|' => {
                        let temp = equation.1[i + 1];
                        let mut factor = 1;
                        while temp >= factor {
                            factor *= 10;
                        }
                        result = result * factor + equation.1[i + 1];
                    }
                    _ => panic!("Invalid operation"),
                }
            }
            if result == equation.0 {
                correct_equations_sum += result;
                break;
            }
        }
    }

    correct_equations_sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().parse::<u64>().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (key, values)
        })
        .collect();
    let operations = ['*', '+'];
    let correct_equations_sum = solve(&operations, equations);

    Some(correct_equations_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap().parse::<u64>().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (key, values)
        })
        .collect();
    let operations = ['*', '+', '|']; // Characters to use

    let correct_equations_sum = solve(&operations, equations);

    Some(correct_equations_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
