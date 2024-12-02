use super::util::*;

fn safe(entries: &Vec<i32>) -> bool {
    let increasing : bool = entries[0] < entries[1];
    for i in 0..entries.len()-1 {
        if increasing {
            if entries[i] >= entries[i+1] || entries[i+1] - entries[i] > 3 {
                return false;
            }
        } else {
            if entries[i] <= entries[i+1] || entries[i] - entries[i+1] > 3 {
                return false;
            }
        }
    }
    true
}

fn part1(input_file: &str) -> i32 {
    let lines = read_lines_in_file(input_file);

    let mut count : i32 = 0;

    'lineloop: for line in lines {
        let entries : Vec<i32> = line.split(" ").map(| item | item.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if safe(&entries) {
            count += 1;
            continue 'lineloop;
        }
    }

    count
}

fn part2(input_file: &str) -> i32 {
    let lines = read_lines_in_file(input_file);

    let mut count : i32 = 0;

    'lineloop: for line in lines {
        let entries : Vec<i32> = line.split(" ").map(| item | item.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if safe(&entries) {
            count += 1;
            continue 'lineloop;
        }

        for i in 0..entries.len() {
            let mut new_entries : Vec<i32> = Vec::<i32>::new();
            for (j, entry) in entries.iter().enumerate() {
                if i == j {
                    continue;
                }
                new_entries.push(*entry);
            }
            if safe(&new_entries) {
                count += 1;
                continue 'lineloop;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day2.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 559);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 23117829);
    }
}
