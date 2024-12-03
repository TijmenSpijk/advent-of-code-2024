advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    
    for line in input.lines() {
        let mut values = line.split_whitespace();
        left.push(values.next().unwrap().parse::<u32>().unwrap());
        right.push(values.next().unwrap().parse::<u32>().unwrap());
    };

    left.sort();
    right.sort();

    Some(
        left.iter().zip(right).fold(0,|a:u32, b| {
            a + b.0.abs_diff(b.1)
        })
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    
    for line in input.lines() {
        let mut values = line.split_whitespace();
        left.push(values.next().unwrap().parse::<u32>().unwrap());
        right.push(values.next().unwrap().parse::<u32>().unwrap());
    };

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
