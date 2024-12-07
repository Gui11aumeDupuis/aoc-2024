advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = deser(input);
    let mut guard_pos = find_gard(&grid)?;
    grid[guard_pos.1][guard_pos.0] = 'X';
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut guard_dir: usize = 0;
    let mut visited: u32 = 1;

    while guard_in_bound(guard_pos.0, guard_pos.1, directions[guard_dir], &grid) {
        let new_guard_pos = (
            (guard_pos.0 as i32 + directions[guard_dir].0) as usize,
            (guard_pos.1 as i32 + directions[guard_dir].1) as usize,
        );

        let step_val = grid[new_guard_pos.1][new_guard_pos.0];

        if step_val == '#' {
            guard_dir = (guard_dir + 1) % 4;
        } else if step_val != 'X' {
            visited += 1;
            grid[new_guard_pos.1][new_guard_pos.0] = 'X';
            guard_pos = (new_guard_pos.0, new_guard_pos.1)
        } else {
            guard_pos = (new_guard_pos.0, new_guard_pos.1)
        }
    }

    Some(visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn deser(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for l in input.lines() {
        let mut row = vec!['0'; l.len()];
        for (i, c) in l.chars().enumerate() {
            row[i] = c;
        }
        grid.push(row);
    }

    grid
}

fn find_gard(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                return Some((x, y));
            }
        }
    }
    None
}

fn guard_in_bound(x: usize, y: usize, direction: (i32, i32), grid: &[Vec<char>]) -> bool {
    let guard_step_x = x as i32 + direction.0;
    let guard_step_y = y as i32 + direction.1;
    if (guard_step_x) < 0
        || (guard_step_x) >= grid[0].len() as i32
        || (guard_step_y) < 0
        || (guard_step_y) >= grid.len() as i32
    {
        return false;
    }

    true
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
        assert_eq!(result, None);
    }
}
