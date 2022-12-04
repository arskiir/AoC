pub fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;

    let mut depths = input.lines();

    let mut prev = depths.next().unwrap().parse::<u32>().unwrap();
    while let Some(depth) = depths.next() {
        let depth = depth.parse::<u32>().unwrap();
        if prev < depth {
            result += 1;
        }
        prev = depth;
    }

    result
}

pub fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;
    let depths: Vec<_> = input.lines().map(|d| d.parse::<u32>().unwrap()).collect();

    let mut prev: u32 = depths.iter().take(3).sum();

    let mut upper_window = 3usize;
    while upper_window < depths.len() {
        let first = depths[upper_window - 2];
        let second = depths[upper_window - 1];
        let third = depths[upper_window];
        let sum = first + second + third;

        if prev < sum {
            result += 1;
        }

        prev = sum;
        upper_window += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1_example_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 1583);
    }

    #[test]
    fn part2_example_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 5);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 1627);
    }
}
