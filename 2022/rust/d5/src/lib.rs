use std::collections::VecDeque;

#[derive(Debug)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

pub fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut lines = stacks.lines().collect::<Vec<_>>();
    let stack_nums = lines.pop().unwrap();
    lines.reverse();

    let mut stacks_count = 0;
    let mut idx_to_stack_idx: Vec<(usize, usize)> = vec![];
    stack_nums.char_indices().for_each(|(i, c)| {
        if c != ' ' {
            let stack_idx = c.to_digit(10).unwrap() - 1;
            idx_to_stack_idx.push((i.try_into().unwrap(), stack_idx.try_into().unwrap()));
            stacks_count += 1;
        }
    });

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];

    lines.iter().for_each(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        for (chars_idx, stack_idx) in idx_to_stack_idx.iter() {
            let c = chars[*chars_idx];
            if c != ' ' {
                stacks[*stack_idx].push(c);
            }
        }
    });

    stacks
}

pub fn parse_moves(moves: &str) -> Vec<Move> {
    moves
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|part| match part.parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                })
                .collect::<Vec<usize>>();
            Move {
                amount: nums[0],
                from: nums[1] - 1,
                to: nums[2] - 1,
            }
        })
        .collect()
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let split = input.split("\n\n").collect::<Vec<_>>();
    (parse_stacks(split[0]), parse_moves(split[1]))
}

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    moves.into_iter().for_each(|mv| {
        for _ in 0..mv.amount {
            let item = stacks[mv.from].pop().unwrap();
            stacks[mv.to].push(item);
        }
    });

    String::from_iter(stacks.into_iter().map(|stack| stack[stack.len() - 1]))
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    let mut items_on_crane: VecDeque<char> = VecDeque::new();

    moves.into_iter().for_each(|mv| {
        for _ in 0..mv.amount {
            let item = stacks[mv.from].pop().unwrap();
            items_on_crane.push_back(item);
        }

        while let Some(item) = items_on_crane.pop_back() {
            stacks[mv.to].push(item);
        }
    });

    String::from_iter(stacks.into_iter().map(|stack| stack[stack.len() - 1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const INPUT: &str = "[N]         [C]     [Z]            
[Q] [G]     [V]     [S]         [V]
[L] [C]     [M]     [T]     [W] [L]
[S] [H]     [L]     [C] [D] [H] [S]
[C] [V] [F] [D]     [D] [B] [Q] [F]
[Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
[P] [P] [C] [W] [W] [F] [W] [J] [C]
[T] [L] [D] [G] [P] [P] [V] [N] [R]
 1   2   3   4   5   6   7   8   9 

move 6 from 2 to 1
move 4 from 6 to 3
move 1 from 6 to 5
move 8 from 3 to 8
move 13 from 8 to 2
move 2 from 7 to 6
move 10 from 1 to 6
move 3 from 2 to 8
move 5 from 4 to 2
move 15 from 6 to 5
move 1 from 1 to 4
move 2 from 7 to 3
move 2 from 4 to 2
move 12 from 5 to 1
move 4 from 8 to 9
move 15 from 1 to 3
move 10 from 9 to 7
move 1 from 5 to 1
move 1 from 4 to 8
move 3 from 7 to 6
move 8 from 2 to 6
move 1 from 9 to 8
move 5 from 2 to 3
move 1 from 4 to 1
move 16 from 3 to 1
move 2 from 2 to 7
move 13 from 1 to 6
move 1 from 2 to 4
move 2 from 2 to 9
move 1 from 4 to 7
move 2 from 8 to 2
move 2 from 2 to 9
move 1 from 6 to 8
move 2 from 3 to 8
move 2 from 1 to 9
move 1 from 3 to 9
move 1 from 3 to 2
move 5 from 5 to 1
move 2 from 9 to 3
move 1 from 2 to 3
move 2 from 1 to 3
move 3 from 3 to 2
move 1 from 5 to 7
move 2 from 7 to 6
move 2 from 8 to 3
move 1 from 8 to 9
move 6 from 3 to 4
move 3 from 9 to 6
move 8 from 6 to 4
move 1 from 2 to 3
move 1 from 2 to 6
move 1 from 2 to 9
move 1 from 3 to 9
move 5 from 9 to 5
move 7 from 7 to 4
move 14 from 4 to 6
move 1 from 5 to 3
move 5 from 1 to 9
move 4 from 5 to 4
move 1 from 1 to 7
move 1 from 3 to 8
move 1 from 8 to 4
move 4 from 9 to 7
move 6 from 6 to 5
move 10 from 4 to 6
move 1 from 9 to 6
move 1 from 4 to 3
move 1 from 3 to 6
move 1 from 4 to 2
move 35 from 6 to 3
move 1 from 2 to 3
move 4 from 5 to 8
move 2 from 5 to 4
move 3 from 8 to 2
move 2 from 4 to 8
move 26 from 3 to 8
move 3 from 2 to 9
move 6 from 3 to 5
move 3 from 5 to 7
move 3 from 7 to 4
move 3 from 4 to 5
move 1 from 9 to 5
move 6 from 5 to 1
move 2 from 8 to 6
move 11 from 8 to 5
move 9 from 5 to 4
move 1 from 9 to 7
move 2 from 7 to 9
move 3 from 1 to 4
move 1 from 5 to 7
move 8 from 6 to 1
move 5 from 7 to 9
move 7 from 9 to 2
move 3 from 2 to 9
move 3 from 7 to 1
move 4 from 9 to 8
move 2 from 5 to 6
move 2 from 2 to 8
move 2 from 6 to 9
move 13 from 8 to 1
move 1 from 2 to 8
move 3 from 3 to 5
move 1 from 9 to 8
move 3 from 5 to 4
move 1 from 9 to 3
move 1 from 2 to 3
move 4 from 8 to 2
move 3 from 2 to 4
move 19 from 1 to 2
move 8 from 1 to 8
move 1 from 4 to 3
move 1 from 4 to 1
move 7 from 2 to 1
move 1 from 3 to 1
move 2 from 3 to 1
move 15 from 4 to 5
move 1 from 1 to 7
move 11 from 2 to 8
move 2 from 2 to 9
move 1 from 3 to 5
move 2 from 9 to 4
move 12 from 8 to 3
move 16 from 5 to 1
move 3 from 4 to 3
move 1 from 7 to 5
move 2 from 8 to 6
move 1 from 5 to 4
move 1 from 4 to 9
move 18 from 1 to 9
move 8 from 3 to 8
move 9 from 8 to 2
move 4 from 9 to 2
move 8 from 1 to 2
move 2 from 6 to 4
move 17 from 2 to 1
move 1 from 4 to 5
move 3 from 2 to 6
move 1 from 2 to 9
move 2 from 6 to 1
move 3 from 3 to 6
move 1 from 4 to 6
move 2 from 3 to 2
move 16 from 9 to 5
move 14 from 5 to 4
move 3 from 5 to 8
move 1 from 2 to 4
move 4 from 8 to 6
move 1 from 2 to 8
move 1 from 3 to 9
move 1 from 3 to 9
move 2 from 9 to 1
move 10 from 8 to 7
move 7 from 6 to 9
move 16 from 1 to 5
move 7 from 4 to 3
move 1 from 8 to 4
move 5 from 4 to 2
move 1 from 5 to 9
move 5 from 9 to 1
move 5 from 1 to 2
move 2 from 9 to 7
move 1 from 1 to 7
move 1 from 6 to 8
move 4 from 4 to 5
move 1 from 6 to 9
move 9 from 2 to 1
move 11 from 5 to 6
move 2 from 9 to 2
move 4 from 3 to 4
move 4 from 4 to 6
move 1 from 3 to 4
move 11 from 7 to 4
move 9 from 4 to 7
move 11 from 7 to 2
move 2 from 3 to 5
move 2 from 4 to 8
move 7 from 5 to 2
move 1 from 8 to 3
move 1 from 5 to 1
move 1 from 3 to 7
move 6 from 2 to 9
move 1 from 8 to 9
move 6 from 9 to 2
move 15 from 6 to 2
move 1 from 7 to 2
move 31 from 2 to 7
move 22 from 7 to 3
move 2 from 5 to 1
move 3 from 7 to 4
move 1 from 4 to 9
move 3 from 4 to 3
move 1 from 8 to 6
move 1 from 9 to 6
move 15 from 1 to 5
move 1 from 9 to 5
move 1 from 1 to 8
move 2 from 6 to 8
move 1 from 8 to 4
move 1 from 4 to 6
move 1 from 6 to 9
move 10 from 3 to 1
move 1 from 9 to 7
move 2 from 7 to 8
move 10 from 5 to 1
move 12 from 1 to 4
move 1 from 3 to 8
move 11 from 4 to 8
move 1 from 8 to 3
move 5 from 5 to 8
move 1 from 5 to 8
move 6 from 8 to 6
move 3 from 2 to 1
move 1 from 6 to 2
move 5 from 1 to 6
move 3 from 1 to 4
move 3 from 2 to 8
move 1 from 2 to 9
move 8 from 3 to 5
move 2 from 1 to 3
move 3 from 7 to 5
move 2 from 3 to 5
move 3 from 5 to 2
move 1 from 7 to 9
move 2 from 9 to 1
move 1 from 6 to 9
move 2 from 4 to 8
move 5 from 6 to 5
move 1 from 6 to 7
move 1 from 9 to 8
move 3 from 6 to 5
move 7 from 8 to 9
move 5 from 9 to 1
move 2 from 4 to 8
move 11 from 5 to 9
move 3 from 2 to 3
move 2 from 5 to 8
move 4 from 3 to 7
move 11 from 9 to 5
move 3 from 7 to 5
move 1 from 3 to 5
move 8 from 1 to 4
move 5 from 3 to 9
move 15 from 5 to 4
move 8 from 4 to 1
move 12 from 8 to 1
move 4 from 5 to 8
move 12 from 4 to 5
move 3 from 7 to 2
move 11 from 5 to 7
move 8 from 8 to 7
move 7 from 9 to 8
move 2 from 5 to 7
move 4 from 7 to 8
move 9 from 8 to 4
move 11 from 4 to 5
move 6 from 7 to 8
move 9 from 8 to 7
move 18 from 7 to 5
move 1 from 8 to 1
move 4 from 1 to 5
move 1 from 7 to 2
move 6 from 1 to 9
move 1 from 2 to 4
move 1 from 4 to 3
move 3 from 1 to 7
move 1 from 4 to 2
move 3 from 2 to 5
move 2 from 9 to 5
move 1 from 2 to 6
move 4 from 7 to 8
move 1 from 6 to 2
move 1 from 2 to 4
move 4 from 8 to 5
move 3 from 9 to 7
move 1 from 9 to 5
move 1 from 4 to 3
move 2 from 3 to 8
move 2 from 7 to 4
move 28 from 5 to 8
move 1 from 8 to 9
move 1 from 9 to 3
move 6 from 5 to 6
move 5 from 5 to 2
move 1 from 3 to 4
move 1 from 7 to 4
move 1 from 5 to 6
move 16 from 8 to 3
move 7 from 1 to 8
move 4 from 4 to 9
move 1 from 2 to 4
move 3 from 2 to 3
move 6 from 6 to 8
move 10 from 3 to 8
move 1 from 2 to 7
move 1 from 6 to 7
move 11 from 8 to 5
move 2 from 7 to 8
move 1 from 1 to 9
move 5 from 9 to 5
move 4 from 3 to 2
move 1 from 4 to 2
move 1 from 3 to 8
move 3 from 8 to 2
move 19 from 8 to 7
move 6 from 7 to 6
move 4 from 5 to 2
move 9 from 7 to 5
move 1 from 7 to 1
move 5 from 6 to 9
move 1 from 7 to 4
move 1 from 6 to 7
move 1 from 4 to 7
move 1 from 1 to 2
move 2 from 7 to 3
move 6 from 5 to 9
move 9 from 9 to 1
move 17 from 5 to 4
move 2 from 3 to 1
move 13 from 4 to 7
move 3 from 3 to 5
move 7 from 1 to 4
move 1 from 5 to 8
move 2 from 5 to 2
move 6 from 7 to 3
move 1 from 5 to 7
move 1 from 9 to 1
move 2 from 3 to 2
move 1 from 9 to 3
move 9 from 7 to 3
move 10 from 3 to 5
move 8 from 4 to 2
move 1 from 4 to 1
move 13 from 2 to 4
move 5 from 4 to 3
move 1 from 5 to 9
move 1 from 7 to 2
move 6 from 4 to 2
move 4 from 1 to 8
move 3 from 4 to 6
move 9 from 8 to 9
move 17 from 2 to 3
move 2 from 8 to 6
move 1 from 4 to 3
move 2 from 6 to 3
move 2 from 1 to 3
move 13 from 3 to 4
move 8 from 9 to 8
move 7 from 4 to 6
move 3 from 5 to 6
move 5 from 8 to 2
move 9 from 6 to 1
move 7 from 2 to 4
move 2 from 6 to 9
move 1 from 1 to 5
move 18 from 3 to 8
move 5 from 1 to 3
move 1 from 6 to 1
move 9 from 4 to 7
move 11 from 8 to 7
move 5 from 7 to 5
move 2 from 4 to 5
move 1 from 6 to 2
move 13 from 7 to 8
move 1 from 4 to 9
move 1 from 9 to 6
move 4 from 1 to 5
move 1 from 7 to 6
move 9 from 5 to 7
move 8 from 5 to 6
move 10 from 7 to 2
move 1 from 5 to 7
move 1 from 7 to 1
move 17 from 8 to 2
move 9 from 6 to 7
move 6 from 7 to 1
move 2 from 7 to 2
move 1 from 4 to 2
move 12 from 2 to 8
move 7 from 1 to 2
move 6 from 8 to 6
move 3 from 8 to 2
move 1 from 7 to 2
move 2 from 3 to 4
move 1 from 4 to 9
move 2 from 3 to 5
move 2 from 3 to 7
move 1 from 4 to 6
move 2 from 7 to 1
move 7 from 2 to 7
move 6 from 7 to 1
move 1 from 5 to 2
move 6 from 8 to 4
move 4 from 9 to 7
move 1 from 5 to 2
move 3 from 8 to 1
move 1 from 9 to 4
move 1 from 7 to 8
move 1 from 8 to 1
move 4 from 7 to 8
move 1 from 4 to 2
move 3 from 6 to 9
move 2 from 9 to 7
move 1 from 9 to 3
move 2 from 4 to 3
move 2 from 8 to 3
move 5 from 3 to 4
move 4 from 6 to 2
move 8 from 2 to 9
move 1 from 6 to 5
move 10 from 2 to 3
move 2 from 8 to 3
move 8 from 9 to 3
move 9 from 2 to 5
move 1 from 2 to 4
move 1 from 2 to 3
move 7 from 5 to 6
move 1 from 5 to 7
move 13 from 3 to 4
move 2 from 7 to 8
move 5 from 3 to 1
move 1 from 5 to 3
move 1 from 8 to 5
move 1 from 2 to 8
move 1 from 7 to 9
move 1 from 4 to 2
move 15 from 4 to 8
move 6 from 4 to 7
move 6 from 7 to 8
move 1 from 6 to 5
move 1 from 4 to 6
move 1 from 9 to 6
move 2 from 5 to 2
move 6 from 6 to 4
move 6 from 1 to 8
move 6 from 4 to 9
move 2 from 6 to 1
move 1 from 2 to 9
move 26 from 8 to 1
move 4 from 3 to 7
move 2 from 2 to 5
move 16 from 1 to 4
move 3 from 9 to 8
move 3 from 8 to 7
move 3 from 5 to 1
move 2 from 9 to 2
move 1 from 9 to 7
move 1 from 9 to 1
move 8 from 4 to 1
move 4 from 4 to 9
move 1 from 2 to 3
move 1 from 3 to 7
move 2 from 8 to 2
move 3 from 4 to 2
move 1 from 4 to 7
move 9 from 7 to 5
move 1 from 9 to 8
move 2 from 9 to 8
move 5 from 5 to 7
move 1 from 9 to 5
move 6 from 2 to 6
move 1 from 8 to 2
move 5 from 6 to 5
move 1 from 7 to 4
move 3 from 8 to 9
move 3 from 9 to 7
move 1 from 6 to 4
move 2 from 4 to 1
move 2 from 5 to 8
move 1 from 2 to 9
move 2 from 8 to 9
move 3 from 9 to 3
move 8 from 7 to 3
move 4 from 5 to 8
move 1 from 3 to 9
move 3 from 5 to 8
move 1 from 5 to 3
move 6 from 8 to 6
move 3 from 3 to 9
move 5 from 3 to 2
move 5 from 6 to 4
move 14 from 1 to 5
move 8 from 5 to 6
move 2 from 3 to 2
move 4 from 9 to 1
move 1 from 8 to 7
move 7 from 2 to 3
move 6 from 3 to 7
move 3 from 5 to 3
move 1 from 3 to 9
move 12 from 1 to 5
move 1 from 9 to 7
move 2 from 3 to 1
move 1 from 7 to 8
move 1 from 8 to 7
move 2 from 3 to 6
move 2 from 1 to 9
move 2 from 5 to 6
move 2 from 9 to 7
move 9 from 7 to 3
move 7 from 1 to 5
move 5 from 5 to 2
move 8 from 6 to 8
move 5 from 8 to 9";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part1_works() {
        // let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(INPUT);
        assert_eq!(result, "SVFDLGLWV");
    }

    #[test]
    fn example_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn part2_works() {
        // let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(INPUT);
        assert_eq!(result, "DCVTCVPCL");
    }
}
