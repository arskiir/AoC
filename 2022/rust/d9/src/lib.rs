use std::collections::BTreeSet;

pub enum Dir {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

pub struct Move {
    pub dir: Dir,
    pub amount: usize,
}

fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().map(|mv| {
        let mut split = mv.split_whitespace();
        let dir = match split.next().unwrap() {
            "U" => Dir::UP,
            "D" => Dir::DOWN,
            "L" => Dir::LEFT,
            _ => Dir::RIGHT,
        };
        let amount = split.next().unwrap().parse().unwrap();
        Move { dir, amount }
    })
}

#[derive(Debug)]
pub struct Head {
    current_pos: (i32, i32),
}

impl Head {
    pub fn new(start_pos: (i32, i32)) -> Self {
        Self {
            current_pos: start_pos,
        }
    }

    pub fn mv(&mut self, dir: &Dir) {
        match dir {
            Dir::UP => self.current_pos.1 += 1,
            Dir::DOWN => self.current_pos.1 -= 1,
            Dir::RIGHT => self.current_pos.0 += 1,
            Dir::LEFT => self.current_pos.0 -= 1,
        }
    }
}

#[derive(Debug)]
pub struct Tail {
    current_pos: (i32, i32),
}

impl Tail {
    pub fn new(start_pos: (i32, i32)) -> Self {
        Self {
            current_pos: start_pos,
        }
    }

    pub fn mv(&mut self, new_head_pos: &(i32, i32), prev_head_pos: (i32, i32)) {
        let x_diff_abs = new_head_pos.0.abs_diff(self.current_pos.0);
        let y_diff_abs = new_head_pos.1.abs_diff(self.current_pos.1);

        // overlap or still touching, do not move
        if self.current_pos == *new_head_pos // overlap
            || (x_diff_abs == 1 && self.current_pos.1.abs_diff(new_head_pos.1) == 0 /* touching horizontally */)
            || (y_diff_abs == 1 && self.current_pos.0.abs_diff(new_head_pos.0) == 0 /* touching vertically */)
            || (x_diff_abs == 1 && y_diff_abs == 1 /* touching diagonally */)
        {
            return;
        }

        self.current_pos = prev_head_pos;
    }
}

pub fn part1(input: &str) -> usize {
    let start_pos = (0, 0); // any position
    let mut head = Head::new(start_pos);
    let mut tail = Tail::new(start_pos);

    let mut pos_tail_visited: BTreeSet<(i32, i32)> = BTreeSet::new();

    for mv in parse_moves(input) {
        for _ in 0..mv.amount {
            let prev_head_pos = head.current_pos;
            head.mv(&mv.dir);
            tail.mv(&head.current_pos, prev_head_pos);
            pos_tail_visited.insert(tail.current_pos);
        }
    }

    pos_tail_visited.len()
}

pub fn part2(input: &str) -> usize {
    let start_pos = (0, 0); // any position
    let mut head = Head::new(start_pos);
    let mut tails: Vec<Tail> = (0..9).map(|_| Tail::new(start_pos)).collect();

    let mut pos_tail_visited: BTreeSet<(i32, i32)> = BTreeSet::new();

    for mv in parse_moves(input) {
        for _ in 0..mv.amount {
            let prev_head_pos = head.current_pos;
            head.mv(&mv.dir);
            let new_head_pos = head.current_pos;

            let mut prev_tail_before_pos = tails[0].current_pos;
            let mut new_tail_before_pos: Option<(i32, i32)> = None;
            for (i, tail) in tails.iter_mut().enumerate() {
                if i == 0 {
                    tail.mv(&new_head_pos, prev_head_pos);
                } else {
                    let prev_pos = tail.current_pos;
                    tail.mv(&new_tail_before_pos.unwrap(), prev_tail_before_pos);
                    prev_tail_before_pos = prev_pos;
                }
                new_tail_before_pos = Some(tail.current_pos);
                // dbg!(prev_tail_before_pos);
                // dbg!(&new_tail_before_pos);
            }
            pos_tail_visited.insert(tails.last().unwrap().current_pos);
            dbg!(&head);
            dbg!(&tails);
        }

        // dbg!(&head);
        // for tail in tails.iter() {
        // dbg!(tail);
        // }
    }

    pos_tail_visited.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn ex_part1_works() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result, 5883);
    }

    #[test]
    fn ex_part2_works() {
        let result = part2(EXAMPLE2);
        assert_eq!(result, 36);
    }
}
