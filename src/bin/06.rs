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

    let mut pos: Pos = Pos {x: 0, y: 0};
    let dir: Direction = Direction::Up( Dir {dx: 0, dy: -1});
    let mut path: HashSet<Pos> = HashSet::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                pos.x = x as i32;
                pos.y = y as i32;
            }
        }
    }

    path.insert(pos);

    Some(move_guard(grid, pos, dir, path))
}

fn move_guard(grid: Vec<Vec<char>>, pos: Pos, dir: Direction, mut path: HashSet<Pos>) -> u32 {

    let dir_values = dir.get_dir();
    let newpos: Pos = Pos { x: pos.x + dir_values.dx, y: pos.y + dir_values.dy };

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
                            move_guard(grid, newpos, dir, path)
                        },
                        '#' => {
                            move_guard(grid, pos, dir.turn(), path)
                        },
                        _ => panic!("Unkown Char Found")
                    }
                },
                None => return path.len() as u32
            }
        },
        None => return path.len() as u32
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // count crossings
    // if you cross line where you have been and path is undisturbed
    // end
    None
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
