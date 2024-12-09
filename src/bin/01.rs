advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_ids: Vec<i32> = Vec::new();
    let mut right_ids: Vec<i32> = Vec::new();
    let input_iter = input.lines();
    let mut total_diff = 0;
    for line in input_iter {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        left_ids.push(left);
        right_ids.push(right);
    }
    left_ids.sort();
    right_ids.sort();
    assert!(left_ids.len() == right_ids.len());

    for i in 0..left_ids.len() {
        let left = left_ids[i];
        let right = right_ids[i];
        let diff = (left - right).abs();
        total_diff += diff;
    }
    Some(total_diff as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_ids: Vec<i32> = Vec::new();
    let mut right_ids: Vec<i32> = Vec::new();
    let input_iter = input.lines();
    let mut similarity_score = 0;
    for line in input_iter {
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        left_ids.push(left);
        right_ids.push(right);
    }

    assert!(left_ids.len() == right_ids.len());

    for id in left_ids.iter() {
        let occurences = right_ids.iter().filter(|&x| x == id).count();
        if occurences > 0 {
            similarity_score += (occurences as i32) * id;
        }
    }
    Some(similarity_score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
