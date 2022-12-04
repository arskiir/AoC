use std::fs;

fn get_input(path: &str) -> String {
    fs::read_to_string(path).expect("Error reading input.txt")
}

pub fn part1(path: &str) -> u32 {
    let input = get_input(path);
    input
        .split("\r\n\r\n")
        .map(|calory_set| {
            calory_set
                .lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part2(path: &str) -> u32 {
    let input = get_input(path);
    let mut result = input
        .split("\r\n\r\n")
        .map(|calory_set| {
            calory_set
                .lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    result.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let result = part1("./example_input.txt");
        assert_eq!(24000, result);
    }

    #[test]
    fn part1_works() {
        let result = part1("./input.txt");
        assert_eq!(65912, result);
    }

    #[test]
    fn part2_works() {
        let result = part2("./input.txt");
        assert_eq!(195625, result);
    }
}
