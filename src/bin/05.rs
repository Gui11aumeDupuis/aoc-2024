use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pagesBefore = HashMap::<&str, Vec<&str>>::new();
    let mut updates = Vec::<Vec<&str>>::new();
    let mut first_part: usize = 0;
    for (i, l) in input.lines().enumerate() {
        if l.is_empty() {
            first_part = i + 1;
            break;
        }

        let (left, right) = l.split_once("|")?;
        let pages = pagesBefore.entry(right).or_default();
        pages.push(left);
    }

    for l in input.lines().skip(first_part) {
        updates.push(l.split(",").collect());
    }

    let mut sum: u32 = 0;
    'up: for update in updates {
        // reset broken rules
        let mut brokenRules = HashMap::<&str, bool>::new();

        let middle = update[(update.len() as u32 / 2) as usize]
            .parse::<u32>()
            .unwrap();
        for page in update {
            if let Some(true) = brokenRules.get(page) {
                continue 'up;
            }

            if let Some(pages) = pagesBefore.get(page) {
                for p in pages {
                    brokenRules.entry(p).or_insert(true);
                }
            }
        }

        sum += middle;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
