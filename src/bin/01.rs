use log::info;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut result: u32 = 0;
    for line in lines {
        let values = line.split(' ');
        let mut difference: i32 = 0;
        let mut counter: i32 = 1;
        for value in values {
            println!("parsing {value}");
            // difference = difference + counter * value.parse::<i32>().expect("error");
            counter = counter * -1;
        }
        result = result + difference.abs() as u32;
    };
    Some(result)
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
