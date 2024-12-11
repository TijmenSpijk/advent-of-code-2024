advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let values: Vec<u32> = line.split_whitespace().map(|a| a.parse::<u32>().unwrap()).collect();
        if check_safety(&values) {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let values: Vec<u32> = line.split_whitespace().map(|a| a.parse::<u32>().unwrap()).collect();
        if check_safety(&values) {
            result += 1;
        } else {
            for i in 0..(values.len()) {
                let mut values_to_check = values.clone();
                values_to_check.remove(i);
                if check_safety(&values_to_check) {
                    result += 1;
                    break;
                }
            }
        }
    }

    Some(result)
}

fn check_safety(values: &Vec<u32>) -> bool {
    let mut dup = values.clone();

    dup.reverse();
    dup.pop();
    dup.reverse();

    let combo = values.iter().zip(dup);

    let increasing = combo.clone().fold(true, |a , b| {
        a && b.0 > &b.1
    });

    let decreasing = combo.clone().fold(true, |a , b| {
        a && b.0 < &b.1
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
