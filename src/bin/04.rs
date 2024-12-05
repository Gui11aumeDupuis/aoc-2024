advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = deser(input);
    let directions: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let height = grid.len();
    let width = grid[0].len();
    let mas = "MAS";
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[x][y] != 'X' {
                continue;
            }

            'dirs: for (dx, dy) in directions {
                let last_x = x as isize + 3 * dx;
                let last_y = y as isize + 3 * dy;
                if last_x < 0 || last_x >= width as isize || last_y < 0 || last_y >= height as isize
                {
                    continue;
                }

                for (m, c) in mas.chars().enumerate() {
                    let n = (m + 1) as isize;
                    let xstep = (x as isize + n * dx) as usize;
                    let ystep = (y as isize + n * dy) as usize;
                    if grid[xstep][ystep] != c {
                        continue 'dirs;
                    }
                }

                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = deser(input);

    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if grid[x][y] != 'A' {
                continue;
            }

            let tl = grid[x - 1][y - 1];
            let tr = grid[x + 1][y - 1];
            let bl = grid[x - 1][y + 1];
            let br = grid[x + 1][y + 1];

            if is_mas_diagonal(tl, br) && is_mas_diagonal(tr, bl) {
                count += 1;
            }
        }
    }

    Some(count)
}

fn is_mas_diagonal(a: char, b: char) -> bool {
    if a == 'M' || a == 'S' {
        let expected = if a == 'M' { 'S' } else { 'M' };
        if b != expected {
            return false;
        }
        return true;
    }

    false
}

fn deser(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for l in input.lines() {
        let mut row = Vec::new();
        for c in l.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
