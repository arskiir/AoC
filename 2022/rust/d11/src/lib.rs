fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\r\n\r\n")
        .map(|split| {
            let mut lines = split.lines();
            lines.next(); // we don't need the monkey number
            lines.map(|l| l.trim_start())
        })
        .enumerate()
        .map(|(id, mut monkey_lines)| {
            let items_line = monkey_lines.next().unwrap();
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
                id,
                inspect_count: 0,
            }
        })
        .collect()
}

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

#[derive(Debug, Clone, Copy)]
pub struct ItemMail {
    pub recipient: usize,
    pub item: i32,
}

#[derive(Debug)]
pub struct Monkey {
    id: usize,
    items: Vec<i32>,
    op: Op,
    divisible_by: i32,
    true_to: usize,
    false_to: usize,
    inspect_count: u32,
}

impl Monkey {
    pub fn take_turn(&mut self) -> Vec<ItemMail> {
        let mut thrown_items = Vec::new();
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
            thrown_items.push(ItemMail {
                item: *item,
                recipient: if *item % self.divisible_by == 0 {
                    self.true_to
                } else {
                    self.false_to
                },
            });
            // increment the inspect count
            self.inspect_count += 1;
        }

        dbg!(&thrown_items);
        thrown_items
    }

    pub fn receive(&mut self, mut items: Vec<ItemMail>) -> Vec<ItemMail> {
        if items.len() != 0 {
            for item in items.iter() {
                if item.recipient == self.id {
                    self.items.push(item.item);
                }
            }
            items = items
                .iter()
                .filter(|item| item.recipient != self.id)
                .map(|item| *item)
                .collect();
        }
        items
    }
}

pub fn part1(input: &str) -> u32 {
    let mut monkeys = parse_input(&input);
    let rounds = 20;
    let mut prev_thrown_items: Vec<ItemMail> = Vec::new();
    for _ in 0..rounds {
        // dbg!(&monkeys);
        for monkey in monkeys.iter_mut() {
            prev_thrown_items = monkey.receive(prev_thrown_items);
            let mut thrown_items = monkey.take_turn();
            prev_thrown_items.append(&mut thrown_items);
        }
    }

    // dbg!(monkeys);
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
