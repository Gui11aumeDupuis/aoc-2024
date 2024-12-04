use std::collections::VecDeque;

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
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let re_dos = Regex::new(r"(do\(\))").unwrap();
    let re_donts = Regex::new(r"(don't\(\))").unwrap();

    let dos: Vec<i32> = re_dos.find_iter(input).map(|m| m.start() as i32).collect();
    let donts: Vec<i32> = re_donts
        .find_iter(input)
        .map(|m| m.start() as i32)
        .collect();

    let mut dos_i = 0;
    let mut donts_i = 0;
    // initialize with a do for firsts multiplications
    let mut conditions: Vec<(i32, bool)> = vec![(-1, true)];

    for _ in 0..dos.len() + donts.len() {
        if dos_i >= dos.len() {
            conditions.push((donts[donts_i], false));
            donts_i += 1;
        } else if donts_i >= donts.len() {
            conditions.push((dos[dos_i], true));
            dos_i += 1;
        } else if dos[dos_i] < donts[donts_i] {
            conditions.push((dos[dos_i], true));
            dos_i += 1;
        } else {
            conditions.push((donts[donts_i], false));
            donts_i += 1;
        }
    }

    let mut sum = 0;
    let mut j = 0;
    let mut i = 0;
    let muls: Vec<(i32, &str)> = re
        .find_iter(input)
        .map(|m| (m.start() as i32, m.as_str()))
        .collect();

    while i < muls.len() {
        if muls[i].0 < conditions[j].0 {
            if j < conditions.len() - 1 {
                j += 1;
            }
            continue;
        } else if j < conditions.len() - 1 && muls[i].0 > conditions[j + 1].0 {
            j += 1;
            continue;
        }

        // at this point, conditions[j] is the closest condition to the left of the mul.
        if conditions[j].1 {
            let (left, right) = &muls[i].1[4..muls[i].1.len() - 1].split_once(",")?;
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            sum += left * right;
        }

        i += 1;
    }

    Some(sum)
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
