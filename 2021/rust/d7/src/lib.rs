pub fn part1(input: &str) -> u32 {
    let positions: Vec<i32> = input.split(',').map(|e| e.parse().unwrap()).collect();
    (positions.iter().min().unwrap().to_owned()..=positions.iter().max().unwrap().to_owned())
        .map(|pos| {
            positions
                .iter()
                .map(|crab_pos| pos.abs_diff(*crab_pos))
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

fn increasing_fuel(from: i32, to: i32) -> u32 {
    let n = from.abs_diff(to) + 1;
    // sum of arithmetic sequence
    ((n as f32) / 2f32 * (n as f32 - 1f32)) as u32
}

pub fn part2(input: &str) -> u32 {
    let positions: Vec<i32> = input.split(',').map(|e| e.parse().unwrap()).collect();
    (positions.iter().min().unwrap().to_owned()..=positions.iter().max().unwrap().to_owned())
        .map(|pos| {
            positions
                .iter()
                .map(|crab_pos| increasing_fuel(*crab_pos, pos))
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 37);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 344735);
    }

    #[test]
    fn example_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 168);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 96798233);
    }
}
