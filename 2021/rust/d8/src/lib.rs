use std::collections::{BTreeSet, HashMap};

pub fn part1(input: &str) -> u32 {
    let numbers: BTreeSet<usize> = [2, 4, 3, 7].into_iter().collect();
    input
        .lines()
        .map(|entry| {
            entry
                .split(" | ")
                .last()
                .unwrap()
                .split_whitespace()
                .map(|d| numbers.contains(&d.len()) as u32)
                .sum::<u32>()
        })
        .sum()
}

fn get_14_lookup(unique10: &str) -> HashMap<u32, BTreeSet<char>> {
    unique10
        .split_whitespace()
        .filter_map(|unique| match unique.len() {
            2 => Some((1, BTreeSet::from_iter(unique.chars()))),
            4 => Some((4, BTreeSet::from_iter(unique.chars()))),
            _ => None,
        })
        .collect()
}

fn decode_single(digit: &str, lookup_1478: &HashMap<u32, BTreeSet<char>>) -> char {
    let len = digit.len();
    match len {
        2 => {
            return '1';
        }
        4 => {
            return '4';
        }
        3 => {
            return '7';
        }
        7 => {
            return '8';
        }
        _ => {}
    }

    let digit_set = BTreeSet::from_iter(digit.chars());
    if len == 6 {
        // 0, 6, or 9
        if lookup_1478
            .get(&4)
            .unwrap()
            .intersection(&digit_set)
            .count()
            == 4
        {
            return '9';
        }
        if lookup_1478
            .get(&1)
            .unwrap()
            .intersection(&digit_set)
            .count()
            == 2
        {
            return '0';
        }
        return '6';
    }

    // 2, 3, or 5
    if lookup_1478
        .get(&1)
        .unwrap()
        .intersection(&digit_set)
        .count()
        == 2
    {
        return '3';
    }

    if lookup_1478
        .get(&4)
        .unwrap()
        .intersection(&digit_set)
        .count()
        == 2
    {
        return '2';
    }

    '5'
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|entry| {
            let mut splits = entry.split(" | ");
            let unique10 = splits.next().unwrap();

            let lookup_14 = get_14_lookup(unique10);
            let four_digits = splits.next().unwrap();
            four_digits
                .split_whitespace()
                .map(|digit| decode_single(digit, &lookup_14))
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn example_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 26);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 342);
    }

    #[test]
    fn example_part2_works() {
        assert_eq!(part2(EXAMPLE), 61229);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part2(&input);
        assert_eq!(result, 1068933);
    }
}
