pub fn part1(input: &str) -> u32 {
    let bit_len = input.lines().next().unwrap().len();
    let data_count = input.lines().count();
    let mut bit_sums = vec![0; bit_len];
    input.lines().for_each(|l| {
        l.char_indices().for_each(|(idx, c)| {
            bit_sums[idx] += c.to_digit(10).unwrap();
        })
    });

    let gamma = bit_sums
        .iter()
        .map(|s| (*s >= (data_count as u32) / 2) as u32);
    let epsilon: String = gamma
        .clone()
        .map(|bit| !(bit != 0) as u32)
        .map(|bit| bit.to_string())
        .collect();
    let gamma: String = gamma.map(|bit| bit.to_string()).collect();

    let gamma_dec = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_dec = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma_dec * epsilon_dec
}

pub fn part2(input: &str) -> u32 {
    let yo = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn o2(
        numbers: &Vec<Vec<u32>>,
        pos: usize,
        numbers_left: usize,
        result: Option<&Vec<u32>>,
    ) -> u32 {
        if numbers_left == 1usize {
            let result: String = result.unwrap().iter().map(|bit| bit.to_string()).collect();
            return u32::from_str_radix(&result, 2).unwrap();
        }

        let bit_sum: u32 = numbers.iter().map(|number| number[pos]).sum();
        let most_common = (bit_sum as f32 >= (numbers_left as f32) / 2f32) as u32;

        let filtered_numbers = numbers
            .clone()
            .into_iter()
            .filter(|number| number[pos] == most_common)
            .collect::<Vec<_>>();

        o2(
            &filtered_numbers,
            pos + 1,
            filtered_numbers.len(),
            Some(&filtered_numbers[0]),
        )
    }

    fn co2(
        numbers: &Vec<Vec<u32>>,
        pos: usize,
        numbers_left: usize,
        result: Option<&Vec<u32>>,
    ) -> u32 {
        if numbers_left == 1usize {
            let result: String = result.unwrap().iter().map(|bit| bit.to_string()).collect();
            return u32::from_str_radix(&result, 2).unwrap();
        }

        let bit_sum: u32 = numbers.iter().map(|number| number[pos]).sum();
        let least_common = !(bit_sum as f32 >= (numbers_left as f32) / 2f32) as u32;

        let filtered_numbers = numbers
            .clone()
            .into_iter()
            .filter(|number| number[pos] == least_common)
            .collect::<Vec<_>>();

        co2(
            &filtered_numbers,
            pos + 1,
            filtered_numbers.len(),
            Some(&filtered_numbers[0]),
        )
    }

    o2(&yo, 0, yo.len(), None) * co2(&yo, 0, yo.len(), None)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 198);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 2743844);
    }

    #[test]
    fn example_part2_works() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 230);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 6677951);
    }
}
