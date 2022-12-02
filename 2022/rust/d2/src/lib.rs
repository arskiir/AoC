use std::fs;

pub fn first() -> u32 {
    let mut score: u32 = 0;

    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let round = line.split(' ').collect::<Vec<_>>();
            let opponent = round[0];
            let you = round[1];

            let move_score = match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            let round_result_score = match (opponent, you) {
                // win
                ("A", "Y") => 6,
                ("B", "Z") => 6,
                ("C", "X") => 6,

                // draw
                ("A", "X") => 3,
                ("B", "Y") => 3,
                ("C", "Z") => 3,

                // lose
                ("A", "Z") => 0,
                ("B", "X") => 0,
                ("C", "Y") => 0,
                _ => 0,
            };

            score += move_score + round_result_score;
        });

    score
}

pub fn second() -> u32 {
    let mut score: u32 = 0;

    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let round = line.split(' ').collect::<Vec<_>>();
            let opponent = round[0];
            let round_outcome = round[1];

            let round_result_score = match round_outcome {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };

            let move_score = match (opponent, round_outcome) {
                // win
                ("A", "Z") => 2,
                ("B", "Z") => 3,
                ("C", "Z") => 1,

                // draw
                ("A", "Y") => 1,
                ("B", "Y") => 2,
                ("C", "Y") => 3,

                // lose
                ("A", "X") => 3,
                ("B", "X") => 1,
                ("C", "X") => 2,
                _ => 0,
            };

            score += move_score + round_result_score;
        });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_works() {
        let result = first();
        assert_eq!(result, 17189);
    }

    #[test]
    fn second_works() {
        let result = second();
        assert_eq!(result, 13490);
    }
}
