use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;

    input.lines().for_each(|racksack| {
        let half_len = racksack.len() / 2;

        let seen_first: HashSet<char> = HashSet::from_iter(racksack[..half_len].chars());
        let seen_second: HashSet<char> = HashSet::from_iter(racksack[half_len..].chars());

        seen_first.intersection(&seen_second).for_each(|t| {
            // A: 65, a: 97
            let ascii = t.clone() as u32;
            if ascii >= 97 {
                result += ascii - 96;
            } else {
                result += ascii - (65 - 27);
            }
        });
    });

    result
}

pub fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;

    let mut lower_window: usize = 0;
    let lines = input.lines().collect::<Vec<_>>();

    while lower_window + 3 <= lines.len() {
        let first: HashSet<char> = HashSet::from_iter(lines[lower_window].chars());
        let second: HashSet<char> = HashSet::from_iter(lines[lower_window + 1].chars());
        let third: HashSet<char> = HashSet::from_iter(lines[lower_window + 2].chars());

        for mutual in first.intersection(&second) {
            if third.contains(mutual) {
                let badge = *mutual;

                // A: 65, a: 97
                let ascii = badge as u32;
                if ascii >= 97 {
                    result += ascii - 96;
                } else {
                    result += ascii - (65 - 27);
                }

                break;
            }
        }

        lower_window += 3;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example_works() {
        let result = part1(EXAMPLE_INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 8233);
    }

    #[test]
    fn second_example_works() {
        let result = part2(EXAMPLE_INPUT);
        assert_eq!(result, 70);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 2821);
    }
}
