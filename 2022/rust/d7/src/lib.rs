pub fn part1(input: &str) -> u32 {
    32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn ex_part1_works() {
        let result = part1("");
        assert_eq!(result, 95437);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 95437);
    }
}
