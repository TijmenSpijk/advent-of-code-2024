advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        if check_safety(line) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        if check_safety(line) {
            result += 1;
        }
    }

    Some(result)
}

fn check_safety(line: &str) -> bool {
    let values = line.split_whitespace().map(|a| a.parse::<u32>().unwrap());
    let mut dup = values.clone();
    dup.next();
    let combo = values.zip(dup);

    let increasing = combo.clone().fold(true, |a , b| {
        a && b.0 > b.1
    });

    let decreasing = combo.clone().fold(true, |a , b| {
        a && b.0 < b.1
    });

    let difference = combo.clone().fold(true, |a , b| {
        a && b.0.abs_diff(b.1) <= 3
    });

    (increasing || decreasing) && difference
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
