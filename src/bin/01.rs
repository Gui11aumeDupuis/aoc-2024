use std::path::absolute;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (l, r) = line.split_once(" ")?;

        match l.parse::<u32>() {
            Ok(i) => left.push(i),
            Err(_) => return None,
        }
        match r.parse::<u32>() {
            Ok(i) => right.push(i),
            Err(_) => return None,
        }
    }

    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
