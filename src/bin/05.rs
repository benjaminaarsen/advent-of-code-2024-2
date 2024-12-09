advent_of_code::solution!(5);


fn check_update(update: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> bool {
    for &page in update.iter() {
        let page_correct = check_page(page, update, page_ordering_rules);
        if !page_correct {
            return false;
        }
    }
    true
}
fn check_update_2(update: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> Vec<i32> {
    if update
        .iter()
        .all(|&page| check_page(page, update, page_ordering_rules))
    {
        return update.clone();
    }

    let mut new_update = update.clone();
    new_update.sort_by(|a, b| {
        for &(rule_a, rule_b) in page_ordering_rules.iter() {
            if rule_a == *a && rule_b == *b {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Greater
    });
    new_update
}

fn check_page(page: i32, update: &Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) -> bool {
    page_ordering_rules
        .iter()
        .filter(|&&(a, _)| a == page)
        .all(|&(a, b)| match (update.contains(&a), update.contains(&b)) {
            (true, true) => {
                update.iter().position(|&x| x == a).unwrap()
                    < update.iter().position(|&x| x == b).unwrap()
            }
            (true, false) | (false, true) => true,
            _ => panic!("impossible"),
        })
}


pub fn part_one(input: &str) -> Option<u32> {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let page_ordering_rules: Vec<(i32, i32)> = data[0]
        .lines()
        .map(|line| {
            let mut iter = line.split("|").map(|x| x.parse::<i32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();
    let updates = data[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total_middle_page_numbers = 0;

    for update in updates.iter() {
        if check_update(update, &page_ordering_rules) {
            total_middle_page_numbers += update.get(update.len() / 2).unwrap();
        }
    }
    Some(total_middle_page_numbers as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let page_ordering_rules: Vec<(i32, i32)> = data[0]
        .lines()
        .map(|line| {
            let mut iter = line.split("|").map(|x| x.parse::<i32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();
    let updates = data[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total_middle_page_numbers = 0;

    for update in updates.iter() {
        let new_update = check_update_2(update, &page_ordering_rules);
        if new_update != *update {
            total_middle_page_numbers += new_update.get(new_update.len() / 2).unwrap();
        }
    }
    Some(total_middle_page_numbers as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
