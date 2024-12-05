use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut products: Vec<(u32, u32)> = Vec::new();
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    for line in input.lines() {
        for (_, [a, b]) in regex.captures_iter(line).map(|c| c.extract()) {
            products.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
        }        
    }

    Some(products.iter().fold(0, | mut a: u32, b | {
        a += b.0 * b.1;
        a
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut products: Vec<(u32, u32)> = Vec::new();
    let regex = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    for line in input.lines() {
        for ins in line.split("do()") {
            let mut split = ins.split("don't()");
            for (_, [a, b]) in regex.captures_iter(split.next().unwrap()).map(|c| c.extract()) {
                products.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
            }
        }
    }

    Some(products.iter().fold(0, | mut a: u32, b | {
        a += b.0 * b.1;
        a
    }))
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