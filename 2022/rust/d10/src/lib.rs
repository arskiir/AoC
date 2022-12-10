pub fn part1(input: &str) -> i32 {
    let mut result = 0;

    let mut reg = 1;
    let mut cycles_passed = 0;
    let cycles_in_question = [20, 60, 100, 140, 180, 220];

    fn process_cycle(
        cycles_passed: &mut i32,
        cycles_in_question: [i32; 6],
        result: &mut i32,
        reg: i32,
    ) {
        *cycles_passed += 1;
        if cycles_in_question.contains(&*cycles_passed) {
            *result += *cycles_passed * reg;
        }
    }

    input
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|mut splits| {
            splits.next().unwrap();
            if let Some(num) = splits.next() {
                // addx operation
                for _ in 0..2 {
                    process_cycle(&mut cycles_passed, cycles_in_question, &mut result, reg);
                }
                let num = num.parse::<i32>().unwrap();
                reg += num;
            } else {
                // noop
                process_cycle(&mut cycles_passed, cycles_in_question, &mut result, reg);
            }
        });

    result
}

pub fn part2(input: &str) {
    let mut reg: i32 = 1;
    let mut cycles_40: i32 = 0;
    let mut screen: Vec<Vec<bool>> = vec![];
    let mut current_row = 0;
    let screen_width = 40;

    fn process_cycle(
        reg: i32,
        cycles_40: &mut i32,
        screen: &mut Vec<Vec<bool>>,
        current_row: &mut usize,
        screen_width: i32,
    ) {
        if *cycles_40 == 0 {
            screen.push(vec![false; screen_width as usize]);
        }
        if reg - 1 <= *cycles_40 && *cycles_40 <= reg + 1 {
            screen[*current_row][*cycles_40 as usize] = true;
        }
        *cycles_40 += 1;
        if *cycles_40 == screen_width {
            *cycles_40 = 0;
            *current_row += 1;
        }
    }

    input
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|mut splits| {
            splits.next().unwrap();
            if let Some(num) = splits.next() {
                // addx operation
                for _ in 0..2 {
                    process_cycle(
                        reg,
                        &mut cycles_40,
                        &mut screen,
                        &mut current_row,
                        screen_width,
                    );
                }
                let num = num.parse::<i32>().unwrap();
                reg += num;
            } else {
                // noop
                process_cycle(
                    reg,
                    &mut cycles_40,
                    &mut screen,
                    &mut current_row,
                    screen_width,
                );
            }
        });

    for row in screen {
        for pixel_on in row {
            if pixel_on {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn ex_part1_works() {
        let example = fs::read_to_string("./example.txt").unwrap();
        let result = part1(&example);
        assert_eq!(result, 13140);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 13060);
    }

    #[test]
    fn ex_part2_works() {
        let input = fs::read_to_string("./example.txt").unwrap();
        part2(&input);
    }

    #[test]
    fn part2_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        part2(&input);
    }
}
