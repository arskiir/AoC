use std::collections::{HashSet, VecDeque};

pub fn solution(input: &str, unique_amount: u32) -> u32 {
    let mut buffer = input.chars();

    let mut unique: HashSet<char> = HashSet::new();

    let mut window: VecDeque<char> = VecDeque::new();
    for _ in 0..unique_amount {
        let c = buffer.next().unwrap();
        unique.insert(c);
        window.push_back(c);
    }

    let mut current_count = unique_amount;
    if unique.len() == unique_amount as usize {
        return current_count;
    }

    while let Some(c) = buffer.next() {
        current_count += 1;
        window.pop_front();
        window.push_back(c);
        let unique: HashSet<char> = HashSet::from_iter(window.clone().into_iter());
        if unique.len() == unique_amount as usize {
            return current_count;
        }
    }

    panic!("Should not reach here");
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn example_part1_works() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(solution(&input, 4), 1855);
    }

    #[test]
    fn example_part2_works() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(solution(&input, 14), 1855);
    }
}
