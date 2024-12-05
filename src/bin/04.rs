advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    let mut result = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                result += find_xmas(&grid, x as i32, y as i32);
            }
        }
    }

    Some(result)
}

fn find_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let mut vec_up: String = "".to_string();
    let mut vec_ur: String = "".to_string();
    let mut vec_right: String = "".to_string();
    let mut vec_dr: String = "".to_string();
    let mut vec_down: String = "".to_string();
    let mut vec_dl: String = "".to_string();
    let mut vec_left: String = "".to_string();
    let mut vec_ul: String = "".to_string();

    let mut strings: Vec<String> = vec![];

    for i in 0..4 {
        let up = (y-i) >= 0;
        let right = (x+i) < grid.len() as i32;
        let down = (y+i) < grid.len() as i32;
        let left = (x-i) >= 0;

        // up
        if up {
            vec_up.push(grid[(y-i) as usize][x as usize]);
        }
        // up right
        if up && right {
            vec_ur.push(grid[(y-i) as usize][(x+i) as usize]);
        }
        // up left
        if up && left {
            vec_ul.push(grid[(y-i) as usize][(x-i) as usize]);
        }
        // right
        if right {
            vec_right.push(grid[y as usize][(x+i) as usize]);
        }
        // down
        if down {
            vec_down.push(grid[(y+i) as usize][x as usize]);
        }
        // down right
        if down && right {
            vec_dr.push(grid[(y+i) as usize][(x+i) as usize]);
        }
        // down left
        if down && left {
            vec_dl.push(grid[(y+i) as usize][(x-i) as usize]);
        }
        // left
        if left {
            vec_left.push(grid[y as usize][(x-i) as usize]);
        }
    }

    strings.push(vec_up);
    strings.push(vec_ur);
    strings.push(vec_right);
    strings.push(vec_dr);
    strings.push(vec_down);
    strings.push(vec_dl);
    strings.push(vec_left);
    strings.push(vec_ul);
    
    strings.iter().fold(0, |mut a, b| {
        if b == "XMAS" {
            a += 1;
        }
        a
    })
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
