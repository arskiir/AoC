pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|p| p.split("-").map(|e| e.parse::<u32>().unwrap()))
        })
        .map(|mut pair| {
            let first_range: Vec<u32> = pair.next().unwrap().collect();
            let second_range: Vec<u32> = pair.next().unwrap().collect();

            if (first_range[0] <= second_range[0] && first_range[1] >= second_range[1])
                || (second_range[0] <= first_range[0] && second_range[1] >= first_range[1])
            {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|p| p.split("-").map(|e| e.parse::<u32>().unwrap()))
        })
        .map(|mut pair| {
            let first_range: Vec<u32> = pair.next().unwrap().collect();
            let second_range: Vec<u32> = pair.next().unwrap().collect();

            if first_range[1] < second_range[0] || second_range[1] < first_range[0] {
                0
            } else {
                1
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 496);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 847);
    }
}
