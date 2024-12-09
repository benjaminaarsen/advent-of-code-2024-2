use std::collections::HashSet;

advent_of_code::solution!(6);

fn is_out_of_bounds(grid: &[Vec<char>], coords: (isize, isize)) -> bool {
    coords.0 < 0
        || coords.0 >= grid.len() as isize
        || coords.1 < 0
        || coords.1 >= grid[0].len() as isize
}

fn rotate_right(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    }
}

fn check_loop(
    grid: &[Vec<char>],
    mut guard: (usize, usize),
    mut guard_direction: (isize, isize),
    obstacle: (usize, usize),
) -> bool {
    let mut visited = HashSet::new();

    loop {
        let (row, col) = guard;

        if !visited.insert((guard, guard_direction)) {
            return true;
        }

        let check_coords = (
            row as isize + guard_direction.0,
            col as isize + guard_direction.1,
        );

        if is_out_of_bounds(grid, check_coords) {
            break;
        }

        if grid[check_coords.0 as usize][check_coords.1 as usize] == '#'
            || obstacle == (check_coords.0 as usize, check_coords.1 as usize)
        {
            guard_direction = rotate_right(guard_direction);
            continue;
        }

        guard = (check_coords.0 as usize, check_coords.1 as usize);
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let guard_start_coords = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == '^').map(|col| (row, col)))
        .unwrap();
    let mut guard_direction = (-1, 0);
    let mut guard = guard_start_coords;
    let mut visited = HashSet::new();

    loop {
        let (row, col) = guard;

        visited.insert(guard);

        let check_coords = (
            row as isize + guard_direction.0,
            col as isize + guard_direction.1,
        );

        if is_out_of_bounds(&grid, check_coords) {
            break;
        }

        if grid[check_coords.0 as usize][check_coords.1 as usize] == '#' {
            guard_direction = rotate_right(guard_direction);
            continue;
        }

        guard = (check_coords.0 as usize, check_coords.1 as usize);
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let guard_start_coords = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == '^').map(|col| (row, col)))
        .unwrap();
    let mut guard_direction = (-1, 0);
    let mut guard = guard_start_coords;
    let mut visited = HashSet::new();
    let mut total_obstacles: u32 = 0;

    loop {
        let (row, col) = guard;
        visited.insert(guard);
        let check_coords = (
            row as isize + guard_direction.0,
            col as isize + guard_direction.1,
        );

        if is_out_of_bounds(&grid, check_coords) {
            break;
        }

        if grid[check_coords.0 as usize][check_coords.1 as usize] == '#' {
            guard_direction = rotate_right(guard_direction);
            continue;
        }

        let obstacle = (check_coords.0 as usize, check_coords.1 as usize);
        if !visited.contains(&obstacle) && check_loop(&grid, guard, guard_direction, obstacle) {
            total_obstacles += 1;
        }

        guard = (check_coords.0 as usize, check_coords.1 as usize);
    }

    Some(total_obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
