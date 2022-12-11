#[derive(Debug)]
enum OpOperand {
    Old,
    Num(i32),
}

#[derive(Debug)]
enum Op {
    Add(OpOperand),
    Mul(OpOperand),
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i32>,
    op: Op,
    divisible_by: i32,
    true_to: u32,
    false_to: u32,
}

impl Monkey {
    pub fn take_turn(&mut self) {
        for item in self.items.iter_mut() {
            // do operation
            match &self.op {
                Op::Add(operand) => match operand {
                    OpOperand::Old => {
                        *item += *item;
                    }
                    OpOperand::Num(num) => {
                        *item += num;
                    }
                },
                Op::Mul(operand) => match operand {
                    OpOperand::Old => {
                        *item *= *item;
                    }
                    OpOperand::Num(num) => {
                        *item *= num;
                    }
                },
            }
            // gets bored
            *item /= 3;
            // throw
            if *item % self.divisible_by == 0 {
            } else {
            }
        }
    }

    pub fn receive(&mut self, items: Vec<i32>) {}
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\r\n\r\n")
        .map(|split| {
            let mut lines = split.lines();
            lines.next(); // we don't need the monkey number line
            lines.map(|l| l.trim_start())
        })
        .enumerate()
        .map(|(monkey_num, mut monkey_lines)| {
            let items_line = monkey_lines.next().unwrap();
            dbg!(&items_line);
            let items = items_line
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect();

            let op_line = monkey_lines.next().unwrap();
            let mut op_line_split = op_line
                .strip_prefix("Operation: new = old ")
                .unwrap()
                .split_whitespace();
            let op_add = op_line_split.next().unwrap() == "+";
            let op_operand = if let Ok(num) = op_line_split.next().unwrap().parse() {
                OpOperand::Num(num)
            } else {
                OpOperand::Old
            };
            let op = if op_add {
                Op::Add(op_operand)
            } else {
                Op::Mul(op_operand)
            };

            let divisible_by_line = monkey_lines.next().unwrap();
            let divisible_by = divisible_by_line
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let true_to = monkey_lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let false_to = monkey_lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                divisible_by,
                true_to,
                false_to,
                op,
            }
        })
        .collect()
    // vec![]
}

pub fn part1(input: &str) -> u32 {
    let mut monkeys = parse_input(&input);
    // for m in monkeys {
    //     dbg!(m);
    // }
    let rounds = 20;
    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            monkey.take_turn();
            monkeys[0].receive(vec![]);
        }
    }
    !todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn ex_part1_works() {
        let example = fs::read_to_string("./example.txt").unwrap();
        let result = part1(&example);
        assert_eq!(result, 10605);
    }
}
