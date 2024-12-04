use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let mut sum = 0;
    for mul in re.find_iter(input).map(|m| m.as_str()) {
        let (left, right) = &mul[4..mul.len() - 1].split_once(",")?;
        let left = left.parse::<u32>().unwrap();
        let right = right.parse::<u32>().unwrap();
        sum += left * right;
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
