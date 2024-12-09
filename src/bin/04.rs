advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_lines = grid.len();
    let n_cols = grid[0].len();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut total = 0;

    for row in 0..n_lines {
        for col in 0..n_cols {
            if grid[row][col] != 'X' {
                continue;
            }
            total += directions.iter().filter(|&&(dx, dy)| {
                (1..=3).all(|step| {
                    let new_row = row as isize + dy * step;
                    let new_col = col as isize + dx * step;
                    if new_row < 0 || new_row >= n_lines as isize || new_col < 0 || new_col >= n_cols as isize {
                        return false;
                    }
                    let expected_char = match step {
                        1 => 'M',
                        2 => 'A',
                        3 => 'S',
                        _ => unreachable!(),
                    };
                    grid[new_row as usize][new_col as usize] == expected_char
                })
            }).count();
        }
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for r in 0..grid.len() - 2 {
        for c in 0..grid[0].len() - 2 {
            let block: [[char; 3]; 3] = [
                [grid[r][c], grid[r][c + 1], grid[r][c + 2]],
                [grid[r + 1][c], grid[r + 1][c + 1], grid[r + 1][c + 2]],
                [grid[r + 2][c], grid[r + 2][c + 1], grid[r + 2][c + 2]],
            ];
            let diag1 = [block[0][0], block[1][1], block[2][2]];
            let diag2 = [block[0][2], block[1][1], block[2][0]];
            if [diag1, diag2]
                .iter()
                .all(|diag| matches!(diag, ['M', 'A', 'S'] | ['S', 'A', 'M']))
            {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
