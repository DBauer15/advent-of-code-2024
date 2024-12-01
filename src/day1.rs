use super::util::*;
use std::collections::HashMap;

fn part1(input_file: &str) -> i32 {
    let lines = read_lines_in_file(input_file);

    let mut leftlist = lines.iter().map(|item| 
        item.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();
    let mut rightlist = lines.iter().map(|item| 
        item.split(' ').collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();

    leftlist.sort();
    rightlist.sort();

    leftlist.iter().zip(rightlist.iter())
                .map(|item| (item.0 - item.1).abs())
                .sum()
}

fn part2(input_file: &str) -> i32 {
    let lines = read_lines_in_file(input_file);

    let mut leftlist = lines.iter().map(|item| 
        item.split(' ').collect::<Vec<&str>>()[0].parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();
    let mut rightlist = lines.iter().map(|item| 
        item.split(' ').collect::<Vec<&str>>().last().unwrap().parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();

    let mut rightcounts = HashMap::new();
    for item in rightlist {
        let value_ref: &mut i32 = rightcounts
            .entry(item)
            .or_insert(0);

        *value_ref += 1;
    }

    leftlist.iter()
                .map(|item| item * rightcounts.get(item).unwrap_or(&0))
                .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day1.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 2756096);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 23117829);
    }
}
