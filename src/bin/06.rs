use std::{collections::HashMap, ops::Add};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars = deser(input);

    let mut lab = Lab::new(&mut chars);
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
    let mut chars = deser(input);

    let mut lab = Lab::new(&mut chars);
    let mut guard = lab.find_gard()?;

    let mut added_obs = 0;

    while guard.in_bound(&lab) {
        let next_pos = guard.get_next_pos();
        let step_val = lab.get(&next_pos);

        if step_val != '#' {
            lab.modify(&next_pos, 'X');
            if lab.obstacle_in_front_would_cause_loop(&guard) {
                added_obs += 1;
            }
            guard.make_step();
        } else {
            if let Some(visits) = lab
                .obstructions
                .get_mut(&(next_pos.x as usize, next_pos.y as usize))
            {
                visits.add_visit(&guard.dir);
            }
            guard.change_direction();
        }
    }

    Some(added_obs)
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
    obs_per_row: Vec<Vec<usize>>,
    obs_per_col: Vec<Vec<usize>>,
    obstructions: HashMap<(usize, usize), ObstructionVisits>,
}

impl<'a> Lab<'a> {
    fn new(grid: &'a mut [Vec<char>]) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let mut obstructions = HashMap::new();

        let mut obs_per_row = Vec::<Vec<usize>>::new();
        for y in 0..height {
            let mut horz_obs = Vec::new();
            for x in 0..width {
                if grid[y][x] == '#' {
                    horz_obs.push(x);
                    obstructions
                        .entry((x, y))
                        .or_insert(ObstructionVisits::new());
                }
            }
            obs_per_row.push(horz_obs)
        }

        let mut obs_per_col = Vec::<Vec<usize>>::new();
        for x in 0..width {
            let mut vert_obs = vec![];
            for y in 0..height {
                if grid[y][x] == '#' {
                    vert_obs.push(y);
                }
            }
            obs_per_col.push(vert_obs)
        }

        Self {
            grid,
            obs_per_row,
            obs_per_col,
            obstructions,
        }
    }

    fn print(&self) {
        println!();
        for row in self.grid.iter() {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }

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

    fn obstacle_in_front_would_cause_loop(&mut self, guard: &Guard) -> bool {
        // find the obstacle to right of guard (based on direction)
        let guard_x = guard.pos.x as usize;
        let guard_y = guard.pos.y as usize;
        let obs = match guard.get_next_dir() {
            Direction::Left => match self.obs_per_row[guard_y]
                .iter()
                .rev()
                .position(|el| *el < guard_x)
            {
                Some(i) => {
                    let idx = self.obs_per_row[guard_y].len() - 1 - i;
                    (self.obs_per_row[guard_y][idx], guard_y)
                }
                None => return false,
            },
            Direction::Right => match self.obs_per_row[guard_y].iter().find(|el| **el > guard_x) {
                Some(x) => (*x, guard_y),
                None => return false,
            },
            Direction::Up => match self.obs_per_col[guard_x]
                .iter()
                .rev()
                .position(|el| *el < guard_y)
            {
                Some(i) => {
                    let idx = self.obs_per_col[guard_x].len() - 1 - i;
                    (self.obs_per_col[guard_x][idx], guard_y)
                }
                None => return false,
            },

            Direction::Down => match self.obs_per_col[guard_x].iter().find(|el| **el > guard_y) {
                Some(y) => (guard_x, *y),
                None => return false,
            },
        };

        if let Some(visits) = self.obstructions.get_mut(&obs) {
            if visits.was_visited(&guard.get_next_dir()) {
                return true;
            }
        }

        false
    }
}

struct Guard {
    pos: Pos,
    dir: Direction,
}

impl Guard {
    fn change_direction(&mut self) {
        self.dir = self.get_next_dir();
    }

    fn get_next_dir(&self) -> Direction {
        match self.dir {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
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

struct ObstructionVisits {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl ObstructionVisits {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
    fn add_visit(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
        }
    }

    fn was_visited(&self, dir: &Direction) -> bool {
        match dir {
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Up => self.up,
            Direction::Down => self.down,
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
        assert_eq!(result, Some(6));
    }
}
