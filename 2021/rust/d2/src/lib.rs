pub fn part1(input: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    input
        .lines()
        .map(|command| command.split_whitespace().collect::<Vec<_>>())
        .for_each(|command| {
            let instruction = command[0];
            let val = command[1].parse::<i32>().unwrap();

            match instruction {
                "forward" => horizontal += val,
                "up" => vertical -= val,
                "down" => vertical += val,
                _ => panic!("unknown instruction"),
            };
        });

    horizontal * vertical
}

pub fn part2(input: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut aim: i32 = 0;

    input
        .lines()
        .map(|command| command.split_whitespace().collect::<Vec<_>>())
        .for_each(|command| {
            let instruction = command[0];
            let val = command[1].parse::<i32>().unwrap();

            match instruction {
                "forward" => {
                    horizontal += val;
                    vertical += aim * val;
                }
                "up" => aim -= val,
                "down" => aim += val,
                _ => panic!("unknown instruction"),
            };
        });

    horizontal * vertical
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 150);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 2147104);
    }

    #[test]
    fn example_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 900);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 2044620088);
    }
}
