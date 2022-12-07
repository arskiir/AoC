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

pub fn part2(input: &str) -> u32 {
    let set4: BTreeSet<char> = BTreeSet::from(['e', 'a', 'f', 'b']);
    let set0: BTreeSet<char> = BTreeSet::from(['c', 'a', 'g', 'e', 'd', 'b']);
    let set6: BTreeSet<char> = BTreeSet::from(['c', 'd', 'f', 'g', 'e', 'b']);
    let set9: BTreeSet<char> = BTreeSet::from(['c', 'e', 'f', 'a', 'b', 'd']);
    dbg!(set4.intersection(&set0).count());
    dbg!(set4.intersection(&set6).count());
    dbg!(set4.intersection(&set9).count());

    32
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
}
