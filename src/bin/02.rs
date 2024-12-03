advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = deser(input);
    let mut safe_count: u32 = 0;
    for r in reports {
        let mut n = r[0];
        let asc = r[1] > r[0];
        safe_count += 1;
        for m in r.iter().skip(1) {
            let diff = m - n;
            if (diff == 0) || (asc && diff < 0) || (!asc && diff > 0) || (diff.abs_diff(0) > 3) {
                safe_count -= 1;
                break;
            }

            n = *m
        }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn deser(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        reports.push(nums);
    }
    reports
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
        assert_eq!(result, None);
    }
}
