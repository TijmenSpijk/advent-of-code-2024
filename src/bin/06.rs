use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Direction {
    Up(Dir),
    Down(Dir),
    Left(Dir),
    Right(Dir)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Dir {
    dx: i32,
    dy: i32
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::Up(_) => Direction::Right(Dir { dx: 1, dy: 0 }),
            Direction::Right(_) => Direction::Down(Dir { dx: 0, dy: 1 }),
            Direction::Down(_) => Direction::Left(Dir { dx: -1, dy: 0 }),
            Direction::Left(_) => Direction::Up(Dir { dx: 0, dy: -1 }),
        }
    }

    pub fn get_dir(&self) -> &Dir {
        match self {
            Direction::Up(dir) => dir,
            Direction::Right(dir) => dir,
            Direction::Down(dir) => dir,
            Direction::Left(dir) => dir,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut guard_pos: Pos = Pos {x: 0, y: 0};
    let dir: Direction = Direction::Up(Dir {dx: 0, dy: -1});
    let mut path: HashSet<Pos> = HashSet::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                guard_pos.x = x as i32;
                guard_pos.y = y as i32;
            }
        }
    }

    path.insert(guard_pos);

    Some(move_guard(&grid, guard_pos, &dir, &mut path).len() as u32)
}

fn move_guard(grid: &Vec<Vec<char>>, guard_pos: Pos, dir: &Direction, path: &mut HashSet<Pos>) -> HashSet<Pos> {

    let dir_values = dir.get_dir();
    let newpos: Pos = Pos { x: guard_pos.x + dir_values.dx, y: guard_pos.y + dir_values.dy };

    match grid.get(newpos.y as usize) {
        Some(line) => {
            match line.get(newpos.x as usize) {
                Some(char) => {
                    match char {
                        '.' => {
                            path.insert(newpos);
                            move_guard(grid, newpos, dir, path)
                        },
                        '^' => {
                            path.insert(newpos);
                            move_guard(grid, newpos, dir, path)
                        },
                        '#' => {
                            move_guard(grid, guard_pos, &dir.turn(), path)
                        },
                        _ => panic!("Unkown Char Found")
                    }
                },
                None => return path.clone()
            }
        },
        None => return path.clone()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut guard_pos: Pos = Pos { x: 0, y: 0 };
    let dir: Direction = Direction::Up(Dir {dx: 0, dy: -1});
    let mut path: HashSet<Pos> = HashSet::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                guard_pos = Pos {x: x as i32, y: y as i32};
            }
        }
    }

    path.insert(guard_pos);
    path = move_guard(&grid, guard_pos, &dir, &mut path);

    let mut count: u32 = 0;

    for obst_pos in path {
        if !(obst_pos == guard_pos) {
            grid[obst_pos.y as usize][obst_pos.x as usize] = '#';
            let mut new_path: HashSet<Pos> = HashSet::new();
            if check_loop(&grid, guard_pos, &dir, &mut new_path) {
                count += 1
            }
            grid[obst_pos.y as usize][obst_pos.x as usize] = '.';
        }
    }

    Some(count)
}

fn check_loop(grid: &Vec<Vec<char>>, guard_pos: Pos, dir: &Direction, path: &mut HashSet<Pos>) -> bool {
    let dir_values = dir.get_dir();
    let newpos: Pos = Pos { x: guard_pos.x + dir_values.dx, y: guard_pos.y + dir_values.dy };

    match grid.get(newpos.y as usize) {
        Some(line) => {
            match line.get(newpos.x as usize) {
                Some(char) => {
                    match char {
                        '.' => {
                            path.insert(newpos);
                            check_loop(grid, newpos, dir, path)
                        },
                        '^' => {
                            path.insert(newpos);
                            check_loop(grid, newpos, dir, path)
                        },
                        '#' => {
                            let new_dir: Direction = dir.turn();
                            let new_dir_values = dir.get_dir();
                            let next: Pos = Pos { x: guard_pos.x + new_dir_values.dx, y: guard_pos.y + new_dir_values.dy };
                            if path.contains(&next) {
                                return true;
                            }
                            check_loop(grid, guard_pos, &new_dir, path)
                        },
                        _ => panic!("Unkown Char Found")
                    }
                },
                None => return false
            }
        },
        None => return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
