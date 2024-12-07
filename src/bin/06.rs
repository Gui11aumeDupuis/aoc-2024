use std::ops::Add;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars = deser(input);

    let mut lab = Lab { grid: &mut chars };
    let mut guard = lab.find_gard()?;

    lab.modify(&guard.pos, 'X');
    let mut visited: u32 = 1;

    while guard.in_bound(&lab) {
        let next_pos = guard.get_next_pos();

        let step_val = lab.get(&next_pos);

        if step_val == '#' {
            guard.change_direction();
        } else if step_val != 'X' {
            visited += 1;
            lab.modify(&next_pos, 'X');
            guard.make_step();
        } else {
            guard.make_step();
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

struct Lab<'a> {
    grid: &'a mut [Vec<char>],
}

impl<'a> Lab<'a> {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn find_gard(&self) -> Option<Guard> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.grid[y][x] == '^' {
                    return Some(Guard {
                        pos: Pos {
                            x: x as i32,
                            y: y as i32,
                        },
                        dir: Direction::Up,
                    });
                }
            }
        }
        None
    }

    fn modify(&mut self, pos: &Pos, val: char) {
        self.grid[pos.y as usize][pos.x as usize] = val;
    }

    fn get(&self, pos: &Pos) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }
}

struct Guard {
    pos: Pos,
    dir: Direction,
}

impl Guard {
    fn change_direction(&mut self) {
        match self.dir {
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
        }
    }

    fn in_bound(&self, lab: &Lab) -> bool {
        let next = self.get_next_pos();
        if (next.x) < 0
            || (next.x) >= lab.width() as i32
            || (next.y) < 0
            || (next.y) >= lab.height() as i32
        {
            return false;
        }

        true
    }

    fn make_step(&mut self) {
        self.pos = self.get_next_pos();
    }

    fn get_next_pos(&self) -> Pos {
        self.pos + self.dir.step()
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn step(&self) -> Pos {
        match *self {
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
            Direction::Up => Pos { x: 0, y: -1 },
            Direction::Down => Pos { x: 0, y: 1 },
        }
    }
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
