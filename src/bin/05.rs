use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (pages_before, updates) = deser(input);

    let mut sum: u32 = 0;
    'up: for update in updates {
        // reset broken rules
        let mut broken_rules = HashMap::<&str, bool>::new();

        let middle = update[(update.len() as u32 / 2) as usize]
            .parse::<u32>()
            .unwrap();
        for page in update {
            if let Some(true) = broken_rules.get(page) {
                continue 'up;
            }

            if let Some(pages) = pages_before.get(page) {
                for p in pages {
                    broken_rules.entry(p).or_insert(true);
                }
            }
        }

        sum += middle;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pages_before, updates) = deser(input);
    let mut sum: u32 = 0;

    for update in updates {
        let (ordered, was_unordered) = order(&update, &pages_before);
        if was_unordered {
            sum += ordered[(ordered.len() as u32 / 2) as usize]
                .parse::<u32>()
                .unwrap();
        }
    }

    Some(sum)
}

fn order<'a>(
    update: &[&'a str],
    pages_before: &'a HashMap<&'a str, Vec<&'a str>>,
) -> (Vec<&'a str>, bool) {
    // reset broken rules
    let mut broken_rules = HashMap::<&str, usize>::new();
    for (idx, page) in update.iter().enumerate() {
        if let Some(pos) = broken_rules.get(*page) {
            // move wrong page before hit
            let mut partially_ordered: Vec<&str> = update.to_vec();
            partially_ordered.remove(idx);
            partially_ordered.insert(*pos, *page);

            // order pages yet to be ordered
            let (ordered, _) = order(&partially_ordered, pages_before);
            return (ordered, true);
        }

        if let Some(pages) = pages_before.get(*page) {
            for p in pages {
                broken_rules.entry(*p).or_insert(idx);
            }
        }
    }

    // if we reach here, recursion is done.
    (update.to_vec(), false)
}

fn deser(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    let mut pages_before = HashMap::<&str, Vec<&str>>::new();
    let mut updates = Vec::<Vec<&str>>::new();
    let mut first_part: usize = 0;
    for (i, l) in input.lines().enumerate() {
        if l.is_empty() {
            first_part = i + 1;
            break;
        }

        let (left, right) = l.split_once("|").unwrap();
        let pages = pages_before.entry(right).or_default();
        pages.push(left);
    }

    for l in input.lines().skip(first_part) {
        updates.push(l.split(",").collect());
    }

    (pages_before, updates)
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
